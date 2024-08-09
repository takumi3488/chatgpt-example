use std::env;

use anyhow::Result;
use ureq::json;

const URL: &str = "https://api.openai.com/v1/chat/completions";

fn main() {
    let prompt = "What is the capital of the United States?";
    let response = get_openai_response(prompt).unwrap();
    println!("{}", response);
}

fn get_openai_response(prompt: &str) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let res: serde_json::Value = ureq::post(URL)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))?
        .into_json()?;

    Ok(res["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .to_string())
}
