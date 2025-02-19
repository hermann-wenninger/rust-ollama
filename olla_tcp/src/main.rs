use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;
//use futures_util::StreamExt; // Import für Stream-Verarbeitung

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn query_ollama(prompt: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://192.168.178.56:11434/api/generate";

    let request_body = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: true, // Falls du Streaming willst, setze es auf true
    };

    let mut response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?;

    let mut full_response = String::new();
    while let Some(chunk) = response.chunk().await? {
        full_response.push_str(&String::from_utf8_lossy(&chunk));
    }

    println!("Raw response: {}", full_response); // Debugging line

    // Deserialize the response
    let ollama_response: serde_json::Value = serde_json::from_str(&full_response)?;
    println!("Deserialized response: {:?}", ollama_response);

    Ok(full_response)
}

#[tokio::main]
async fn main() {
    match query_ollama("Pleace write me a vector sort in rust", "deepseek-coder").await {
        Ok(answer) => println!("Antwort: {}", answer),
        Err(e) => eprintln!("Fehler: {}", e),
    }
}
//a webapp in rust with the framework actix in backend and react in the frontend to login,logout,register,update users and there profiles all with jwtokens for the authentifier prozess in best practices