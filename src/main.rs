mod config;
mod joke;
mod llm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = config::Config::load()?;

    println!("Hello, world!");
    println!("2 + 2 = {}", add(2, 2));

    let model = llm::OpenAiChatModel::new(&cfg.llm.base_url, &cfg.llm.api_key, &cfg.llm.model);
    let joke = joke::tell_me_a_joke(&model, "Tell me a joke").await?;
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
