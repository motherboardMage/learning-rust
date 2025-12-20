use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const FULL_ENTROPY_THRESHOLD: usize = 100_000;
const GUESS_SAMPLE_SIZE: usize = 4000;
const ENTROPY_ESTIMATION_SAMPLE_SIZE: usize = 8000;
const ALPHANUMERIC_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterSet {
    Numeric,
    Alphanumeric,
}

impl CharacterSet {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "numeric" => Ok(CharacterSet::Numeric),
            "alphanumeric" => Ok(CharacterSet::Alphanumeric),
            _ => Err(format!(
                "Invalid character set '{}'. Choose 'numeric' or 'alphanumeric'.",
                s
            )),
        }
    }

    fn get_chars(&self) -> &[u8] {
        match self {
            CharacterSet::Numeric => &ALPHANUMERIC_CHARS[0..10],
            CharacterSet::Alphanumeric => ALPHANUMERIC_CHARS,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            CharacterSet::Numeric => "Numeric (0-9)",
            CharacterSet::Alphanumeric => "Alphanumeric (0-9, a-z)",
        }
    }
}

/// Configuration for the game session
pub struct Config {
    digits: usize,
    char_set: CharacterSet,
}

impl Config {
    pub fn parse() -> Result<Self, String> {
        let mut digits = 4;
        let mut char_set = CharacterSet::Numeric;
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-d" | "--digits" => {
                    let val_str = args
                        .next()
                        .ok_or_else(|| "Expected a value for --digits flag.".to_string())?;
                    let n: usize = val_str
                        .parse()
                        .map_err(|_| format!("Invalid value for --digits: '{}'.", val_str))?;
                    if n > 0 && n <= 8 {
                        digits = n;
                    } else {
                        return Err(format!(
                            "Number of digits must be between 1 and 8, but got {}.",
                            n
                        ));
                    }
                }
                "-c" | "--chars" => {
                    let val_str = args
                        .next()
                        .ok_or_else(|| "Expected a value for --chars flag.".to_string())?;
                    char_set = CharacterSet::from_str(&val_str)?;
                }
                _ => {}
            }
        }

        Ok(Config { digits, char_set })
    }

    pub fn digits(&self) -> usize {
        self.digits
    }

    pub fn char_set(&self) -> CharacterSet {
        self.char_set
    }
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new() -> Self {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Lcg { state: start }
    }

    fn next_usize(&mut self, max: usize) -> usize {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state as usize) % max
    }
}

pub struct Solver {
    candidates: Vec<Vec<u8>>,
    rng: Lcg,
    char_set: CharacterSet,
    digits: usize,
    total_search_space: usize,
    guess_history: Vec<(Vec<u8>, usize)>,
}

impl Solver {
    pub fn new(config: &Config) -> Result<Self, String> {
        let digits = config.digits();
        let char_set = config.char_set();

        println!(
            "[*] Initializing search space for {} characters ({})...",
            digits,
            char_set.name()
        );

        let search_space_size = if char_set == CharacterSet::Alphanumeric && digits > 0 {
            let num_base: usize = 10;
            let alpha_base: usize = 26;
            num_base.checked_pow((digits - 1) as u32).unwrap_or(0) * alpha_base
        } else {
            let base = char_set.get_chars().len();
            base.checked_pow(digits as u32)
                .ok_or("Search space size would overflow")?
        };

        if digits > 6 && char_set == CharacterSet::Alphanumeric {
            return Err(format!(
                "Search space is too large for {}-character alphanumeric codes. Maximum is 6.",
                digits
            ));
        }
        if digits > 7 && char_set == CharacterSet::Numeric {
            return Err(format!(
                "Search space is too large for {}-digit numeric codes. Maximum is 7.",
                digits
            ));
        }

        let candidates = Self::generate_search_space(digits, char_set);
        println!(
            "[*] Search space initialized with {} candidates.",
            candidates.len()
        );

        Ok(Solver {
            candidates,
            rng: Lcg::new(),
            char_set,
            digits,
            total_search_space: search_space_size,
            guess_history: Vec::new(),
        })
    }

    pub fn candidates(&self) -> &[Vec<u8>] {
        &self.candidates
    }
    pub fn total_search_space(&self) -> usize {
        self.total_search_space
    }

    fn generate_search_space(digits: usize, char_set: CharacterSet) -> Vec<Vec<u8>> {
        if char_set == CharacterSet::Alphanumeric && digits > 0 {
            let numeric_chars = &ALPHANUMERIC_CHARS[0..10];
            let alpha_chars = &ALPHANUMERIC_CHARS[10..];
            let num_base = numeric_chars.len();
            let alpha_base = alpha_chars.len();

            if digits == 0 {
                return vec![];
            }

            let num_permutations = num_base.pow((digits.saturating_sub(1)) as u32);

            (0..num_permutations)
                .flat_map(|i| {
                    let mut num_part = vec![0u8; digits.saturating_sub(1)];
                    let mut num = i;
                    for j in (0..digits.saturating_sub(1)).rev() {
                        num_part[j] = (num % num_base) as u8;
                        num /= num_base;
                    }

                    (0..alpha_base).map(move |k| {
                        let mut code = num_part.clone();
                        code.push(10 + k as u8);
                        code
                    })
                })
                .collect()
        } else {
            let chars = char_set.get_chars();
            let base = chars.len();
            (0..base.pow(digits as u32))
                .map(|i| {
                    let mut num = i;
                    let mut code = vec![0u8; digits];
                    for j in (0..digits).rev() {
                        code[j] = (num % base) as u8;
                        num /= base;
                    }
                    code
                })
                .collect()
        }
    }

    fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
        a.iter().zip(b.iter()).filter(|&(x, y)| x != y).count()
    }

    fn calculate_matches(secret: &[u8], guess: &[u8]) -> usize {
        secret
            .iter()
            .zip(guess.iter())
            .filter(|&(s, g)| s == g)
            .count()
    }

    pub fn prune(&mut self, guess: &[u8], actual_matches: usize) {
        let initial_count = self.candidates.len();
        self.candidates
            .retain(|cand| Self::calculate_matches(cand, guess) == actual_matches);
        let removed = initial_count - self.candidates.len();
        println!(
            "[*] Pruned {} candidates. {} remaining.",
            removed,
            self.candidates.len()
        );
    }

    pub fn add_guess_to_history(&mut self, guess: &[u8], matches: usize) {
        self.guess_history.push((guess.to_vec(), matches));
    }

    pub fn vec_to_string(&self, vec: &[u8]) -> String {
        let chars = self.char_set.get_chars();
        vec.iter().map(|&d| chars[d as usize] as char).collect()
    }

    pub fn string_to_vec(&self, s: &str) -> Option<Vec<u8>> {
        let chars = self.char_set.get_chars();
        let char_map: HashMap<u8, u8> = chars
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i as u8))
            .collect();

        if s.len() == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(s.len());
        for c in s.bytes() {
            match char_map.get(&c) {
                Some(&val) => result.push(val),
                None => return None, // Invalid character found
            }
        }
        Some(result)
    }

    pub fn get_best_guess_idx(&mut self) -> Option<usize> {
        if self.candidates.is_empty() {
            return None;
        }
        if self.candidates.len() == 1 {
            return Some(0);
        }

        let start_time = Instant::now();
        let best_idx = if self.candidates.len() > FULL_ENTROPY_THRESHOLD {
            println!(
                "[*] Candidate pool is large ({}). Using sampled approximation for speed.",
                self.candidates.len()
            );
            self.approximate_best_guess_entropy()
        } else {
            println!("[*] Evaluating optimal guess using parallel entropy calculation...");
            self.get_best_guess_entropy_parallel()
        };
        let duration = start_time.elapsed();
        println!(
            "[*] Found best guess in {:.2?} using Entropy strategy.",
            duration
        );

        best_idx
    }

    fn get_best_guess_entropy_parallel(&self) -> Option<usize> {
        let n_candidates = self.candidates.len() as f64;
        let max_matches = self.digits + 1;
        let pb = ProgressBar::new(self.candidates.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let (best_idx, _) = self
            .candidates
            .par_iter()
            .enumerate()
            .progress_with(pb)
            .map(|(idx, guess)| {
                let mut outcome_counts = vec![0; max_matches];
                for secret in &self.candidates {
                    let m = Self::calculate_matches(secret, guess);
                    outcome_counts[m] += 1;
                }

                let entropy = outcome_counts
                    .iter()
                    .filter(|&&count| count > 0)
                    .map(|&count| {
                        let p = count as f64 / n_candidates;
                        -p * p.log2()
                    })
                    .sum::<f64>();
                (idx, entropy)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;

        Some(best_idx)
    }

    fn approximate_best_guess_entropy(&mut self) -> Option<usize> {
        let n_candidates = self.candidates.len();
        let max_matches = self.digits + 1;

        // Step 1: Sample the guesses using Tournament Selection
        let guess_sample: Vec<_> = if self.guess_history.is_empty() {
            println!("[*] No guess history. Using random sampling for guesses.");
            (0..GUESS_SAMPLE_SIZE)
                .map(|_| {
                    let idx = self.rng.next_usize(n_candidates);
                    (idx, self.candidates[idx].clone())
                })
                .collect()
        } else {
            println!("[*] Using tournament selection to find promising guesses.");
            let mut best_guesses = self.guess_history.clone();
            best_guesses.sort_by(|a, b| b.1.cmp(&a.1));
            let top_guides: Vec<_> = best_guesses.iter().take(5).map(|(g, _)| g).collect();

            (0..GUESS_SAMPLE_SIZE)
                .map(|_| {
                    let idx1 = self.rng.next_usize(n_candidates);
                    let idx2 = self.rng.next_usize(n_candidates);
                    let cand1 = &self.candidates[idx1];
                    let cand2 = &self.candidates[idx2];

                    let fitness1 = top_guides
                        .iter()
                        .map(|g| Self::hamming_distance(cand1, g))
                        .min()
                        .unwrap_or(self.digits + 1);
                    let fitness2 = top_guides
                        .iter()
                        .map(|g| Self::hamming_distance(cand2, g))
                        .min()
                        .unwrap_or(self.digits + 1);

                    if fitness1 <= fitness2 {
                        (idx1, cand1.clone())
                    } else {
                        (idx2, cand2.clone())
                    }
                })
                .collect()
        };

        // Step 2: Sample the secrets for entropy estimation
        let secret_sample: Vec<_> = (0..ENTROPY_ESTIMATION_SAMPLE_SIZE)
            .map(|_| self.candidates[self.rng.next_usize(n_candidates)].clone())
            .collect();

        let pb = ProgressBar::new(guess_sample.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        // Step 3 & 4: Calculate approximate entropy and find the best guess
        let (best_idx, _) = guess_sample
            .par_iter()
            .progress_with(pb)
            .map(|(original_idx, guess)| {
                let mut outcome_counts = vec![0; max_matches];
                for secret in &secret_sample {
                    let m = Self::calculate_matches(secret, guess);
                    outcome_counts[m] += 1;
                }

                let entropy = outcome_counts
                    .iter()
                    .filter(|&&count| count > 0)
                    .map(|&count| {
                        let p = count as f64 / ENTROPY_ESTIMATION_SAMPLE_SIZE as f64;
                        -p * p.log2()
                    })
                    .sum::<f64>();

                (*original_idx, entropy)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;

        Some(best_idx)
    }
}
