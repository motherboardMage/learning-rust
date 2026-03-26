use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> io::Result<String> {
    let mut username = String::new();

    File::open("username.txt")
        .or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("username.txt")
            } else {
                Err(error)
            }
        })?
        .read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let _username = match read_username_from_file() {
        Ok(name) => name,
        Err(_) => "".to_string(),
    };
}
