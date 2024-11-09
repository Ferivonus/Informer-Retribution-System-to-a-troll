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
                    ==                     ==
                 <^\()/^>               <^\()/^>
                  \/  \/                 \/  \/
                   /__\      .  '  .      /__\ 
      ==            /\    .     |     .    /\            ==
   <^\()/^>       !_\/       '  |  '       \/_!       <^\()/^>
    \/  \/     !_/I_||  .  '   \'/   '  .  ||_I\_!     \/  \/
     /__\     /I_/| ||      -== + ==-      || |\_I\     /__\
     /_ \   !//|  | ||  '  .   /.\   .  '  || |  |\\!   /_ \
    (-   ) /I/ |  | ||       .  |  .       || |  | \I\ (=   )
     \__/!//|  |  | ||    '     |     '    || |  |  |\\!\__/
     /  \I/ |  |  | ||       '  .  '    *  || |  |  | \I/  \
    {_ __}  |  |  | ||                     || |  |  |  {____}
 _!__|= ||  |  |  | ||   *      +          || |  |  |  ||  |__!_
 _I__|  ||__|__|__|_||          A          ||_|__|__|__||- |__I_
 -|--|- ||--|--|--|-||       __/_\__  *    ||-|--|--|--||= |--|-
  |  |  ||  |  |  | ||      /\-'o'-/\      || |  |  |  ||  |  |
  |  |= ||  |  |  | ||     _||:<_>:||_     || |  |  |  ||= |  |
  |  |- ||  |  |  | || *  /\_/=====\_/\  * || |  |  |  ||= |  |
  |  |- ||  |  |  | ||  __|:_:_[I]_:_:|__  || |  |  |  ||- |  | 
 _|__|  ||__|__|__|_||:::::::::::::::::::::||_|__|__|__||  |__|_
 -|--|= ||--|--|--|-||:::::::::::::::::::::||-|--|--|--||- |--|-
  jgs|- ||  |  |  | ||:::::::::::::::::::::|| |  |  |  ||= |  | 
~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^~~~~~~~~~
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
