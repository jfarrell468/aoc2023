use anyhow::Result;
use util::input_lines;

fn part1(lines: &Vec<String>) -> Result<u32> {
    Ok(0)
}

fn part2(lines: &Vec<String>) -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    let lines = input_lines();
    println!("Part 1: {}", part1(&lines)?);
    println!("Part 2: {}", part2(&lines)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn lines() -> Vec<String> {
        indoc! {"
            // TODO: example input
        "}
        .lines()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&lines())?, 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&lines())?, 0);
        Ok(())
    }
}
