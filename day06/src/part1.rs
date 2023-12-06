pub struct race {
    pub time: u32,
    pub record_distance: u32
}

impl race {
    pub fn new(time: u32, distance: u32) -> Self {
        Self { time, record_distance: distance }
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut races: Vec<race> = vec![];

    // PARSE
    let mut lines = input.lines();
    let mut strip_line = lines.next().unwrap().strip_prefix("Time: ").expect("Error on parse times");
    let times: Vec<u32> = strip_line.split(" ").filter_map(|x| x.trim().parse::<u32>().ok()).collect();

    strip_line = lines.next().unwrap().strip_prefix("Distance: ").expect("Error on parse distances");
    let dists: Vec<u32> = strip_line.split(" ").filter_map(|x| x.trim().parse::<u32>().ok()).collect();

    for i in 0..times.len() {
        races.push(race::new(times[i], dists[i]))
    }

    // function for interval x = |0, time|
    let mut sums: Vec<u32> = vec![];
    for r in races{
        let mut win: u32 = 0;

        for x in 0..r.time {
            let dist = x * (r.time - x);
            if dist > r.record_distance {
                win += 1;
            }
        }
        sums.push(win);
    }

    Ok(sums.iter().product::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
                assert_eq!("288", process(input)?);
                Ok(())
    }
}