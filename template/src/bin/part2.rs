use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
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