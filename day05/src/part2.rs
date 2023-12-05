use prse::parse;
use crate::part1::seed_map;

pub fn process(input: &str) -> anyhow::Result<String> {
    // parse each map into its own hashmap
    // only add the ranges as a key for each, when we "query" the maps, we use or(input), to return source as destination as required if outside map range

    let mut seeds: Vec<u64> = vec![];

    // 0: seed-to-soil map:
    // 1: soil-to-fertilizer map:
    // 2: fertilizer-to-water map:
    // 3: water-to-light map:
    // 4: light-to-temperature map:
    // 5: temperature-to-humidity map:
    // 6: humidity-to-location map:
    let mut maps: Vec<Vec<seed_map>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),];

    // now, we want to split out input up into blocks for each map. Can we split entire input by newlines?
    for (i, map) in input.split("\r\n\r\n").enumerate() {
        // first one is seeds, throw them into an vector
        if i == 0 {
            let mut seed_parsed: Vec<u64> = vec![];
            parse!(map, "seeds: {seed_parsed: :}");

            for x in (0..seed_parsed.len()).step_by(2) {
                for y in 0..seed_parsed[x+1] {
                    seeds.push(seed_parsed[x]+y);
                }
            }

            continue;
        }
        // map each map, they are given in order
        // skip the first

        for (n, line) in map.split("\r\n").enumerate() {
            // skip title
            if n == 0 { continue};

            let (d_range, s_range, range): (u64, u64, u64) = parse!(line, "{0} {1} {2}");

            maps[i-1].push(seed_map::new(d_range, s_range, range));
        }

    }

    // todo the real solution here is likely to pick the lowest start point of locations, and then calculate backwards from location range until we get a start seed that is valid.
    //  in that way we avoid part of the brute force....
    //  but running in release mode, and it took about 2-3min to solve it, I'll leave it at this for now.

    // now go through all seeds and convert them to location. Lets save their location in a new vector with same index as seeds. Although should probably make a struct for seeds and
    //  note down each id per map, for easier search...
    let mut seed_location:Vec<u64> = vec![];
    println!("seed_size: {:?}", seeds.len());

    for (num, seed) in seeds.iter().enumerate() {
        let mut next_id = *seed;
        // go through all maps
        for map in maps.iter() {
            // go through every range of map, and find the one you fit into
            for r in map.iter() {
                if next_id >= r.source_range && next_id < (r.source_range + r.range) {
                    let debug: u64 = (next_id - r.source_range);
                    next_id = r.dest_range + (next_id - r.source_range);
                    // println!("seed: {:?}, next_minus_source: {:?}, source_start: {:?}, dest_start: {:?}, new_id: {:?}", seed, debug, r.source_range, r.dest_range, next_id);
                    break;
                }
            }
        }
        seed_location.push(next_id);
    }

    // println!("{:?}", seed_location);
    Ok(seed_location.iter().min().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let file = include_str!("../test.txt");
        assert_eq!("46", process(file)?);
        Ok(())
    }
}