use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}

// On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;
    for line in input.lines() {

        let numbers:Vec<u32> = line.chars().filter_map(|x| x.to_digit(10)).collect();
        let value_string = format!("{:?}{:?}", numbers[0], numbers[numbers.len()-1]);
        println!("{}", value_string);
        let value = value_string.parse::<u32>()?;
        sum += value;

    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}