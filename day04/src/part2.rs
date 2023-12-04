use std::collections::{HashMap};

pub fn process(input: &str) -> anyhow::Result<String> {
    //  Every id loaded into a hashmap? with a vector of winning numbers. this gives a data structure that have an id of every single win on every single card
    //  Then maybe use a queue to push each new winning id onto, until its empty. And a sum increasing on every card we process. When queue is done, then we have been through all cards and know how many cards
    //  we have been though?
    let mut map:HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        // get id
        let parse_string = line.strip_prefix("Card ").unwrap();
        // split on : for actual id
        let (id_string, parse_string) = parse_string.rsplit_once(": ").unwrap();
        let id:u32 = id_string.trim().parse()?;

        // split winning and loosing numbers
        let (win_num, own_num) = parse_string.split_once("|").unwrap();
        let win_vec: Vec<u32> = win_num.split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect();
        let own_vec: Vec<u32> = own_num.split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect();

        let matches:u32 = own_vec.iter().map(|x| if win_vec.contains(x) {1} else {0}).sum();
        // add wins to map under this id
        map.insert(id, matches);
    }

    // now we have a hashmap of winnings, time to go through each ticket and add to our queue
    let mut queue:Vec<u32> = vec![];
    // initial population of queue
    for (id, val) in map.iter() {
        for n in 1..val+1 {
            queue.push(id+n);
        }
    }

    let mut sum:u32 = 0;
    // add initial cards we were given
    sum += map.values().len() as u32;

    // resolve queue
    while !queue.is_empty() {
        let id = queue.pop().unwrap();
        sum += 1;

        // add wins on that id, if it exists
        let val:u32 = map[&id];
        for n in 1..val+1 {
            queue.push(id+n);
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}