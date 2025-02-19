use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use futures::{SinkExt, StreamExt};
use warp::ws::Message;
use ollama_rs::OllamaClient;

#[tokio::main]
async fn main() {
    let ollama_client = Arc::new(Mutex::new(OllamaClient::new("http://192.167.178.56:11434")));
    
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_ollama_client(ollama_client.clone()))
        .map(|ws: warp::ws::Ws, client| {
            ws.on_upgrade(move |socket| handle_socket(socket, client))
        });

    let routes = ws_route.with(warp::cors().allow_any_origin());
    
    println!("WebSocket Server running on ws://127.0.0.1:3030/ws");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_ollama_client(client: Arc<Mutex<OllamaClient>>)
    -> impl Filter<Extract = (Arc<Mutex<OllamaClient>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

async fn handle_socket(ws: warp::ws::WebSocket, client: Arc<Mutex<OllamaClient>>) {
    let (mut tx, mut rx) = ws.split();
    
    while let Some(Ok(msg)) = rx.next().await {
        if let Ok(text) = msg.to_str() {
            let response = process_with_ollama(text, client.clone()).await;
            if tx.send(Message::text(response)).await.is_err() {
                break;
            }
        }
    }
}

async fn process_with_ollama(input: &str, client: Arc<Mutex<OllamaClient>>) -> String {
    let client = client.lock().await;
    match client.generate_completion("model_name", input).await {
        Ok(response) => response.text.unwrap_or_else(|| "No response".to_string()),
        Err(_) => "Error communicating with Ollama".to_string(),
    }
}
