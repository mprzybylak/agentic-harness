use async_openai::{
    config::OpenAIConfig,
    types::chat::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

pub async fn tell_me_a_joke(
    base_url: &str,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
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
