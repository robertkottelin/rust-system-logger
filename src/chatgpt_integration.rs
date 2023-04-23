use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Serialize)]
struct RequestData {
    prompt: String,
}

#[derive(Deserialize)]
struct ResponseData {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "your_openai_api_key";
    let url = "https://api.openai.com/v1/engines/davinci-codex/completions";
    let client = Client::new();

    let mut x = 1;
    loop {
        // read system logs on ubuntu
        let mut file = File::open("/var/log/syslog").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            if line.contains("error") {
                let prompt = format!("Summarize the following error and suggest a fix: {}", line);

                let request_data = RequestData { prompt };
                let response = client
                    .post(url)
                    .header("Authorization", format!("Bearer {}", api_key))
                    .json(&request_data)
                    .send()
                    .await?
                    .json::<ResponseData>()
                    .await?;

                let summary_and_fix = response.choices.get(0).map(|c| &c.text).unwrap_or("");
                println!(
                    "Error found: {}\nSummary and fix: {}",
                    line, summary_and_fix
                );
            }
        }

        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
        // increment x
        x += 1;

        if x == 2 {
            break;
        }
    }

    Ok(())
}
