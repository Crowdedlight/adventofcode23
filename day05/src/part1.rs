use std::collections::HashMap;
use prse::parse;

pub struct seed_map {
    pub dest_range: u64,
    pub source_range: u64,
    pub range: u64
}
impl seed_map {
    pub fn new(dest_range: u64, source_range: u64, range: u64) -> Self {
        Self { dest_range, source_range, range }
    }
}

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
            parse!(map, "seeds: {seeds: :}");
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

    // now go through all seeds and convert them to location. Lets save their location in a new vector with same index as seeds. Although should probably make a struct for seeds and
    //  note down each id per map, for easier search...
    let mut seed_location:Vec<u64> = vec![];

    for seed in seeds {
        let mut next_id = seed;
        // go through all maps
        for map in maps.iter() {

            // go through every range of map, and find the one you fit into
            for r in map.iter() {
                if next_id >= r.source_range && next_id < (r.source_range + r.range) {
                    let debug: u64 = (next_id - r.source_range);
                    next_id = r.dest_range + (next_id - r.source_range);
                    println!("seed: {:?}, next_minus_source: {:?}, source_start: {:?}, dest_start: {:?}, new_id: {:?}", seed, debug, r.source_range, r.dest_range, next_id);
                    break;
                }
            }
        }
        seed_location.push(next_id);
    }

    println!("{:?}", seed_location);
    Ok(seed_location.iter().min().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
                assert_eq!("35", process(input)?);
                Ok(())
    }
}