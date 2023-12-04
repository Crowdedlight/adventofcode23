pub fn process(input: &str) -> anyhow::Result<String> {
    // todo
    //  Every id loaded into a hashmap? with a vector of winning numbers. this gives a data structure that have an id of every single win on every single card
    //  Then maybe use a queue to push each new winning id onto, until its empty. And a sum increasing on every card we process. When queue is done, then we have been through all cards and know how many cards
    //  we have been though?


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!()
    }
}