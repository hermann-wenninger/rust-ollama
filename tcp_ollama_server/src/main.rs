use reqwest::blocking::Client;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let ollama_url = "http://192.168.178.56:11434/api/generate";

    let response = client.post(ollama_url)
        .json(&json!({"model": "deepseek-coder", "prompt": "Hello, how are you?"}))
        .send()?
        .text()?;

    println!("Response: {}", response);
    Ok(())
}