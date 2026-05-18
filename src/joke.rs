use crate::llm::{ChatModel, LlmError, Message, Role};

/// Asks the model for a joke using `prompt` and returns its reply.
///
/// # Errors
/// Returns an error if the underlying chat call fails.
pub async fn tell_me_a_joke<C: ChatModel>(client: &C, prompt: &str) -> Result<String, LlmError> {
    let messages = [
        Message {
            role: Role::System,
            content: "You are a stand-up comedian. Reply with one short, clean joke.".to_owned(),
        },
        Message {
            role: Role::User,
            content: prompt.to_owned(),
        },
    ];
    let response = client.chat(&messages).await?;
    Ok(response.content)
}
