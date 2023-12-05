pub use self::adjacent::Adjacent;
mod adjacent;

pub fn input_lines() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
}
