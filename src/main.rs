use reqwest::blocking::Client;
use serde::Serialize;
use std::{thread, time::Duration};

static WEBHOOK_URL: &str = "https://discord.com/api/webhooks/1246739033208651826/QYhEfh8u5hjMRWODBi_rv8IaMXdIkPwf_GX7fhPYE6UfP1EBcOhxo3Ow1nNb4R2ZEk-2";

#[derive(Serialize)]
struct DiscordMessage {
    content: String,
}

fn send_discord_message(content: &str) {
    let client = Client::new();
    let message = DiscordMessage {
        content: content.to_string(),
    };

    let response = client.post(WEBHOOK_URL).json(&message).send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Gönderim başarılı: {}", content);
            } else {
                eprintln!("Gönderim başarısız, HTTP status: {}", res.status());
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
        }
    }
}

fn main() {
    let poem = r#"
    https://cdn.discordapp.com/attachments/751757196164857867/1250499003544633426/WhatsApp_Gorsel_2024-06-12_saat_01.31.25_80ec2846.jpg?ex=6730ea73&is=672f98f3&hm=c0638e8b2b9140c0d80e7ea15fb377d854493c89dfd69c2ac56108444c059e85&
"#;

    for _ in 0..10 {
        send_discord_message(poem);
    }

    for _ in 0..1000 {
        for _ in 0..15 {
            send_discord_message(poem);
        }
        let twenty_seconds = Duration::from_secs(25);
        println!("I finish some");
        thread::sleep(twenty_seconds);
    }

    println!("I finished");
}
