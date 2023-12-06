
pub fn process(input: &str) -> anyhow::Result<String> {

    // PARSE
    let mut lines = input.lines();
    let mut strip_line = lines.next().unwrap().strip_prefix("Time: ").expect("Error on parse times");
    let times: String = strip_line.split(" ").filter(|x| !x.trim().is_empty()).collect();
    let time: f64 = times.trim().parse()?;

    strip_line = lines.next().unwrap().strip_prefix("Distance: ").expect("Error on parse distances");
    let dists: String = strip_line.split(" ").filter(|x| !x.trim().is_empty()).collect();
    let dist: f64 = dists.trim().parse()?;


    // function for interval x = |0, time|
    // let dist_func = x * (time - x);

    // function for intersect
    // let cutoff = 0*x + dist;

    // theory, set the functions equal each other, so
    // dist = x * (time - x)
    // dist = (-x + time)*x
    // x1 = (-t + sqrt(t^2-4d)) / 2
    // x2 = (-t - sqrt(t^2-4d)) / 2
    let x1: f64 = (-time + (time.powf(2.0) - 4.0*dist).sqrt()) / 2.0;
    let x2: f64 = (-time - (time.powf(2.0) - 4.0*dist).sqrt()) / 2.0;
    // println!("x1: {:?}, x2: {:?}", x1, x2);
    //
    let sum = x2.abs() - x1.abs();
    Ok((sum as u64).to_string())

    // Stupid way
    // let mut win: u64 = 0;
    //
    // for x in 0..(time as u64) {
    //     let dist_curr = x * (time as u64 - x);
    //     if dist_curr > dist as u64 {
    //         win += 1;
    //     }
    // }

    // Ok(win.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}