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

    let mut results:Vec<u64> = vec![];

    // go through each start branch and run the pattern until we find first node ending with z
    for start in map.keys().filter(|x| x.ends_with("A")) {

        // going
        let mut sum = 0;
        let mut queue: VecDeque<char> = instructions.chars().collect();

        // add start
        let mut key = start;

        while !queue.is_empty() {
            // check end condition
            if key.ends_with("Z") {
                results.push(sum);
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
    }

    // As each branch will loop with same length, we need to find the least common number they go into
    // using list multiple, starting with biggest value
    // TODO not an optimal solution for least common multiple, should really have implemented one of the division or prime factorization methods
    let mut i = *results.iter().max().unwrap();
    let max = i.clone();
    loop {

        let modulo_test = results.iter().filter(|&x| i % *x == 0).copied().collect::<Vec<u64>>();
        if modulo_test.len() == results.len() {
            // we got our result, as every result element has % i == 0
            break;
        }
        // try next i
        i += max;
    }
    Ok(i.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
                assert_eq!("6", process(input)?);
                Ok(())
    }
}