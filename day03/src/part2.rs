pub fn process(input: &str) -> anyhow::Result<String> {
    // todo okay, same system as part1. But only looking for "*" symbol now. If we find that, we save position of *, into a hashmap as key, with vector of u32 and push our number into it.
    //  Then at end we take all keys that have a vector of length 2, and sum it all up. Should give us result as two numbers would share same "*" coordinates. 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!()
    }
}