mod config;
mod joke;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load()?;

    println!("Hello, world!");
    println!("2 + 2 = {}", add(2, 2));

    let joke = joke::tell_me_a_joke(&cfg.llm.base_url).await?;
    println!("Joke: {joke}");

    Ok(())
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn add_handles_negatives() {
        assert_eq!(add(-1, 1), 0);
    }
}
