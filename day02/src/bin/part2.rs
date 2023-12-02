use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut sum:u32 = 0;

    for line in input.lines() {
        let (_, rest_string) = line.split_once(":").unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        // parse rounds
        let rounds = rest_string.split(";");
        for val in rounds {
            // split per ,
            let balls = val.split(",");
            for ball in balls {
                // get amount
                let (amount, _) = ball.trim().split_once(" ").unwrap();
                let amount_parsed = amount.parse::<u32>().unwrap();

                // match balls
                match ball {
                    x if x.contains("red") => {if amount_parsed > max_red {max_red = amount_parsed}},
                    x if x.contains("green") => {if amount_parsed > max_green {max_green = amount_parsed}},
                    x if x.contains("blue") => {if amount_parsed > max_blue {max_blue = amount_parsed}},
                    _ => {}
                }
            }
        }
        // get sum of balls for possibility
        sum += max_red * max_blue * max_green;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}