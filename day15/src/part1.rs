pub fn process(input: &str) -> anyhow::Result<String> {
    let data_split : Vec<&str> = input.split(",").collect();

    let mut result: Vec<u32> = vec![];

    for &data in data_split.iter() {
        result.push(hash(data));
    }

    let sum: u32 = result.iter().sum();
    Ok(sum.to_string())
}

pub fn hash(input: &str) -> u32 {
    let mut current_value = 0u32;
    for char in input.chars() {
        let char_code = char as u32;
        current_value += char_code;
        current_value *= 17;
        current_value = current_value % 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
                assert_eq!("1320", process(input)?);
                Ok(())
    }
}