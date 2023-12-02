use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}

// struct Round {
//     pub red: u32,
//     pub green: u32,
//     pub blue: u32,
// }
// struct Game {
//     pub id: u32,
//     pub rounds: Vec<Round>
// }
// impl Game {
//     pub fn new(id:u32, red:u32, green:u32, blue:u32) -> Game {
//         Game {id, red, green, blue}
//     }
// }

pub fn process(input: &str) -> anyhow::Result<String> {
    // first have to parse input
    // let games: Vec<Game> = vec![];

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum:u32 = 0;

    for line in input.lines() {
        let (id_string, rest_string) = line.split_once(":").unwrap();
        let (_, id) = id_string.split_once(" ").unwrap();

        // impossible?
        let mut impossible = false;

        // parse rounds
        let rounds = rest_string.split(";");
        for val in rounds {
            // split per ,
            let balls = val.split(",");
            for ball in balls {
                // 3 blue, 4 red
                let mut compare_max = 0;
                match ball {
                    x if x.contains("red") => {compare_max = max_red},
                    x if x.contains("green") => {compare_max = max_green},
                    x if x.contains("blue") => {compare_max = max_blue},
                    _ => {}
                }
                // get amount
                let (amount, _) = ball.trim().split_once(" ").unwrap();
                let amount_parsed = amount.parse::<u32>().unwrap();
                // check if possible
                if amount_parsed > compare_max {
                    impossible = true;
                    break;
                }
            }
        }

        // if any showing was impossible, we breaked and thus should just continue
        if impossible {
            continue;
        }
        // otherwise we add id to sum
        sum += id.parse::<u32>().unwrap();
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}