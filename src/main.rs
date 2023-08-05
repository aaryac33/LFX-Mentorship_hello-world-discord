use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};
use anyhow::Result;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message is from a bot or in a channel (not a DM)
        if msg.author.bot || msg.is_private() {
            return;
        }

        // Extract the question from the message content
        let question = &msg.content;

        // Send the question to the flow function-backed HTTP service
        let response = match send_question_to_http_service(question).await {
            Ok(answer) => answer,
            Err(_) => "Error processing the question.".to_string(),
        };

        // Send the response to the user
        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}

async fn send_question_to_http_service(question: &str) -> Result<String> {
    // Replace the URL below with the URL of your flow function-backed HTTP service
    let url = "https://example.com/ask";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&json!({
            "question": question,
        }))
        .send()
        .await?;

    let body = response.text().await?;
    let json_response: serde_json::Value = serde_json::from_str(&body)?;

    // Assuming the response contains an "answer" field in JSON
    if let Some(answer) = json_response.get("answer") {
        if let Some(answer_str) = answer.as_str() {
            return Ok(answer_str.to_string());
        }
    }

    Ok("Error: Invalid response from the HTTP service.".to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let discord_token = std::env::var("DISCORD_TOKEN")?;

    let mut client = Client::new(&discord_token)
        .event_handler(Handler)
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
