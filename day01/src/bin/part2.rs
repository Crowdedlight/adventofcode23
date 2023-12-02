#![feature(test)]
extern crate test;

use anyhow::Context;
pub fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;

    // okay, so first we need to translate any numbers written out, into digits. So we should make a new string we copy into?
    // let re = Regex::new(r"(?<one>one)|(?<two>two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(ten)|(\d)").unwrap();

    for line in input.lines() {

        let mut line_filtered: String = line.to_string();
        // need to find matches. If reuse of same letters then only first one is valid, like eightwothree == 83
        let mut search_string:Vec<char> = vec![];

        for char in line.chars() {
            // add char to search string
            search_string.push(char);

            match search_string.iter().collect::<String>().to_lowercase() {
                x if x.contains("one") => {line_filtered = line_filtered.replacen("one", "1", 1); search_string.clear();}
                x if x.contains("two") => {line_filtered = line_filtered.replacen("two", "2", 1); search_string.clear();}
                x if x.contains("three") => {line_filtered = line_filtered.replacen("three", "3", 1); search_string.clear();}
                x if x.contains("four") => {line_filtered = line_filtered.replacen("four", "4", 1); search_string.clear();}
                x if x.contains("five") => {line_filtered = line_filtered.replacen("five", "5", 1); search_string.clear();}
                x if x.contains("six") => {line_filtered = line_filtered.replacen("six", "6", 1); search_string.clear();}
                x if x.contains("seven") => {line_filtered = line_filtered.replacen("seven", "7", 1); search_string.clear();}
                x if x.contains("eight") => {line_filtered = line_filtered.replacen("eight", "8", 1); search_string.clear();}
                x if x.contains("nine") => {line_filtered = line_filtered.replacen("nine", "9", 1); search_string.clear();}
                _ => {}
            }
        }

        let numbers:Vec<u32> = line_filtered.chars().filter_map(|x| x.to_digit(10)).collect();
        let value_string = format!("{:?}{:?}", numbers[0], numbers[numbers.len()-1]);
        println!("{} - {}", value_string, line);
        let value = value_string.parse::<u32>()?;
        sum += value;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }

    #[bench]
    fn bench_process(b: &mut Bencher) {
        // exact code to benchmark must be passed as a closure to the iter
        // method of Bencher
        let file = include_str!("../../input2.txt");
        b.iter(|| process(file));
    }
}