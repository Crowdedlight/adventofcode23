use std::collections::HashMap;
use std::collections::VecDeque;
use prse::parse;
pub fn process(input: &str) -> anyhow::Result<String> {

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut instructions = "";
    // parse input
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            instructions = line.clone();
            continue;
        };

        if line.is_empty() {continue};

        // parse string
        let (node, left_child, right_child): (&str, &str, &str) = parse!(line, "{0} = ({1}, {2})");
        // add to hashmap
        map.insert(node, (left_child, right_child));
    }

    let mut sum = 0;
    let mut queue: VecDeque<char> = instructions.chars().collect();

    // add start
    let mut key = "AAA";

    while !queue.is_empty() {
        // check end condition
        if key == "ZZZ" {
            break;
        }

        // get node
        let (l_child, r_child) = map.get(key).unwrap();

        let step = queue.pop_front().expect("Popping on empty queue should not happen");

        if step == 'L' {
            key = l_child;
        } else {
            key = r_child;
        }

        // if queue is empty, we add instructions again to repeat pattern
        if queue.is_empty() {
            queue = instructions.chars().collect();
        }

        // taken a new step
        sum += 1;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
                assert_eq!("2", process(input)?);
                Ok(())
    }
    #[test]
    fn test_process_two() -> anyhow::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}