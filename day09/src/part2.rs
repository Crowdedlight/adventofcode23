fn get_next(nums: Vec<i64>) -> i64 {
    // make new vector based on difference
    let mut vec:Vec<i64> = vec![];
    for i in 0..(nums.len()-1) {
        vec.push(nums[i+1] - nums[i])
    }

    let zero_count = vec.iter().filter(|&x| *x == 0i64).copied().collect::<Vec<i64>>();
    if zero_count.len() == vec.len() {
        return 0;
    } else {
        let next_val = vec.first().unwrap() - get_next(vec.clone());
        return next_val;
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut result: Vec<i64> = vec![];
    for line in input.lines() {
        // get into vector of u64
        let start_row: Vec<i64> = line.split(" ").map(|x| x.trim().parse::<i64>().expect("Error parsing u64 input")).collect();
        let next_val = start_row.first().unwrap() - get_next(start_row.clone());
        result.push(next_val);
    }
    Ok(result.iter().sum::<i64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
                assert_eq!("2", process(input)?);
                Ok(())
    }
}