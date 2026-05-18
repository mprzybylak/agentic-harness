use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

pub type LlmError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

pub trait ChatModel {
    /// Sends `messages` to the model and returns the assistant reply.
    ///
    /// # Errors
    /// Returns an error if the underlying provider call fails or the model
    /// returns no content.
    async fn chat(&self, messages: &[Message]) -> Result<Message, LlmError>;
}

pub struct OpenAiChatModel {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiChatModel {
    #[must_use]
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let config = OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(api_key);
        Self {
            client: Client::with_config(config),
            model: model.to_owned(),
        }
    }
}

fn build_system_message(content: &str) -> Result<ChatCompletionRequestMessage, LlmError> {
    Ok(ChatCompletionRequestSystemMessageArgs::default()
        .content(content)
        .build()?
        .into())
}

fn build_user_message(content: &str) -> Result<ChatCompletionRequestMessage, LlmError> {
    Ok(ChatCompletionRequestUserMessageArgs::default()
        .content(content)
        .build()?
        .into())
}

fn build_assistant_message(content: &str) -> Result<ChatCompletionRequestMessage, LlmError> {
    Ok(ChatCompletionRequestAssistantMessageArgs::default()
        .content(content)
        .build()?
        .into())
}

impl ChatModel for OpenAiChatModel {
    async fn chat(&self, messages: &[Message]) -> Result<Message, LlmError> {
        let request_messages: Vec<ChatCompletionRequestMessage> = messages
            .iter()
            .map(|msg| match msg.role {
                Role::System => build_system_message(&msg.content),
                Role::User => build_user_message(&msg.content),
                Role::Assistant => build_assistant_message(&msg.content),
            })
            .collect::<Result<_, _>>()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .messages(request_messages)
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .ok_or("model returned no content")?;

        Ok(Message {
            role: Role::Assistant,
            content,
        })
    }
}
