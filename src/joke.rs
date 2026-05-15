use async_openai::{
    config::OpenAIConfig,
    types::chat::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

pub async fn tell_me_a_joke() -> Result<String, Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new()
        .with_api_base("http://localhost:11434/v1")
        .with_api_key("ollama");

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("qwen2.5:7b")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("Tell me a joke")
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await?;
    let joke = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or("model returned no content")?;

    Ok(joke)
}
