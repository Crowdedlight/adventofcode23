use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}

pub fn process(input: &str) -> anyhow::Result<String> {
    !todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> anyhow::Result<()> {
        !todo!()
    }
}