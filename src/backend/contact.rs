use actix_web::{post, web, HttpResponse, Responder, get};
use serde::{Deserialize, Serialize};
use crate::pages;
use reqwest::Client;


#[derive(Serialize, Deserialize, Debug)]
struct ContactFormData {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[get("/contact")]
pub async fn get_contact() -> impl Responder {
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Contact));
}

async fn send_discord_dm(form: ContactFormData) -> Result<(), Box<dyn std::error::Error>> {
    let bot_token = std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let user_id = std::env::var("DISCORD_USER_ID").expect("Expected an ID in the environment");

    let client = Client::new();
    let dm_channel_resp = client
        .post("https://discord.com/api/v10/users/@me/channels")
        .header("Authorization", format!("Bot {}", bot_token))
        .json(&serde_json::json!({ "recipient_id": user_id }))
        .send()
        .await?;

    if !dm_channel_resp.status().is_success() {
        let error_text = dm_channel_resp.text().await?;
        eprintln!("Failed to create DM channel: {}", error_text);
        return Err(format!("Failed to create DM channel: {}", error_text).into());
    }

    let dm_channel: serde_json::Value = dm_channel_resp.json().await?;

    let channel_id = match dm_channel["id"].as_str() {
        Some(id) => id,
        None => {
            eprintln!("Error: DM channel 'id' field not present in response.");
            return Err("DM channel 'id' field not present in response.".into());
        }
    };

    let message_content = format!(
        "**New Contact Form Submission**\n**Name:** {}\n**Email:** {}\n**Subject:** {}\n**Message:**\n{}",
        form.name, form.email, form.subject, form.message
    );

    let send_message_resp = client
        .post(&format!(
            "https://discord.com/api/v10/channels/{}/messages",
            channel_id
        ))
        .header("Authorization", format!("Bot {}", bot_token))
        .json(&serde_json::json!({ "content": message_content }))
        .send()
        .await?;

    if send_message_resp.status().is_success() {
        Ok(())
    } else {
        let error_text = send_message_resp.text().await?;
        Err(format!("Failed to send message: {}", error_text).into())
    }
}

#[post("/contact")]
pub async fn submit_contact(form: web::Form<ContactFormData>) -> impl Responder {
    match send_discord_dm(form.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Thank you for reaching out! Your message has been sent."),
        Err(e) => {
            eprintln!("Error sending Discord message: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to send your message. Please try again later.")
        }
    }
}
