use coding_problems::input_with_exit;

fn main() {
    let input = input_with_exit("Enter numbers seperated by spaces: ", true);

    let mut nums: Vec<i32> = Vec::new();

    for str in input.split_whitespace() {
        let num = match str.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number, skipping it.", str);
                continue;
            }
        };

        nums.push(num);
    }

    nums.sort();
    let result = stats(&nums);

    println!(
        "Mean of array is {}\nMedian of array is {}",
        result.0, result.1
    );

    match result.2 {
        None => println!("No mode exists for the provided numbers!"),
        Some(num) => println!("Mode of array is {num}"),
    }
}

fn stats(nums: &[i32]) -> (f64, f64, Option<i32>) {
    if nums.is_empty() {
        return (0.0, 0.0, None);
    }

    (
        calculate_mean(nums),
        calculate_median(nums),
        calculate_mode(nums),
    )
}

fn calculate_mean(nums: &[i32]) -> f64 {
    nums.iter().sum::<i32>() as f64 / nums.len() as f64
}

fn calculate_median(nums: &[i32]) -> f64 {
    let med_idx: usize = (nums.len() - 1) / 2;
    if nums.len() % 2 == 0 {
        return (nums[med_idx] + nums[med_idx + 1]) as f64 / 2.0;
    } else {
        return nums[med_idx] as f64;
    }
}

fn calculate_mode(nums: &[i32]) -> Option<i32> {
    let mut max = 1;
    let mut curr = 1;
    let mut mode = nums[0];

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            curr += 1;
        } else {
            if curr > max {
                max = curr;
                mode = nums[i - 1];
            }
            curr = 1;
        }
    }

    if curr > max {
        max = curr;
        mode = nums[nums.len() - 1]
    }

    match (max, nums.len()) {
        (1, 1) => Some(nums[0]),
        (1, _) => None,
        (_, _) => Some(mode),
    }
}
