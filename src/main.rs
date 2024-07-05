use tokio;
use std::env;
use reqwest::{Client};
use select::document::Document;
use select::predicate::Any;
use std::error::Error;
use serde::{Serialize, Deserialize};

mod my_secret;
use crate::my_secret::API_KEY_VAR;

#[derive(Serialize, Deserialize)]
pub struct TagValuePair {
    tag: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

async fn call_openai_chat(
    system_prompt: &str,
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error>> {
    // Create the request body
    let request_body = ChatRequest {
        model: "gpt-4-turbo".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let chat_response: ChatResponse = response.json().await?;
        if let Some(choice) = chat_response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err("No choices in response".into())
        }
    } else {
        let error_message = response.text().await?;
        Err(error_message.into())
    }
}

// pub async fn fetch_and_extract(url: &str, tags: Vec<&str>) -> Result<Vec<TagValuePair>, Box<dyn Error>> {
//     // Create a reqwest client with SOCKS5 proxy pointing to TOR proxy port
//     let proxy = Proxy::all("socks5h://127.0.0.1:9050")?;
//     let client = Client::builder()
//         .proxy(proxy)
//         .build()?;



// pub async fn fetch_and_extract(url: &str, tags: Vec<&str>, proxy_url: &str) -> Result<Vec<TagValuePair>, Box<dyn Error>> {
//     // Create a reqwest client with the provided proxy
//     let proxy = reqwest::Proxy::all(proxy_url)?;
//     let client = Client::builder()
//         .proxy(proxy)
//         .build()?;

pub async fn fetch_and_extract(url: &str, tags: Vec<&str>, proxy_url: Option<&str>) -> Result<Vec<TagValuePair>, Box<dyn Error>> {
    let client = if let Some(proxy_url) = proxy_url {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };
    println!("{:?}", client);
    // Fetch the content from the URL
    let res = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    // Parse the HTML document
    let document = Document::from(res.as_str());

    // Extract the contents of the specified tags or paths
    let mut results = Vec::new();
    for node in document.find(Any) {
        if let Some(tag) = node.name() {
            for &tag_spec in &tags {
                if tag_spec.contains('.') {
                    let parts: Vec<&str> = tag_spec.split('.').collect();
                    if parts.len() == 2 && tag == parts[0] {
                        if let Some(attr_value) = node.attr(parts[1]) {
                            results.push(TagValuePair {
                                tag: tag_spec.to_string(),
                                value: attr_value.to_string(),
                            });
                        }
                    }
                } else if tag == tag_spec {
                    results.push(TagValuePair {
                        tag: tag.to_string(),
                        value: node.text(),
                    });
                }
            }
        }
    }

    Ok(results)
}

async fn information_extraction(input: &str, entities: Option<&[&str]>, api_key_var: &str, proxy_url: Option<&str>) -> Result<String, Box<dyn Error>> {
    // Handle optional entities
    let entities_list = match entities {
        Some(list) => list,
        None => &[],
    };

    let text_blob;
    // Determine if input is a URL or a text blob
    if input.starts_with("http://") || input.starts_with("https://") {
        // Fetch and extract tags from the URL
        let tags = vec!["h1", "h2", "h3", "h4", "p", "article", "td", "ul", "li", "lo", "a", "a.href"];
        let results = fetch_and_extract(input, tags, proxy_url).await?;
        // Concatenate all tag values into a single text blob
        text_blob = results.iter().map(|result| result.value.clone()).collect::<Vec<String>>().join(" ");
    } else {
        // Use the input directly as the text blob
        text_blob = input.to_string();
    }

    // Create the prompt for GPT
    let entities_str = if entities_list.is_empty() {
        "<no entities provided>".to_string()
    } else {
        entities_list.join(", ")
    };

    let constructed_prompt = format!(
        "extract the following entities from this article and return a JSON with edges and nodes, both lists of objects. Each object in the nodes JSON list can have the keys (value, type) and valid types are {}. The list of edges each have keys (from, to, type) where from and to are node values and type is a verblike word or phrase.\n\n{}",
        entities_str, text_blob
    );

    // Call OpenAI Chat
    let response = call_openai_chat("You are a helpful assistant.", &constructed_prompt, api_key_var).await?;
    Ok(response)
}

#[tokio::main]
async fn main() {

    // Retrieve command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <input> [entities_comma_separated]");
        return;
    }

    let input = &args[1];
    let entities: Option<Vec<&str>> = if args.len() > 2 {
        Some(args[2].split(',').collect())
    } else {
        None
    };

    let proxy_url: Option<&str> = None;
    // let proxy_url: &str = "socks5h://127.0.0.1:9050";
    match information_extraction(input, entities.as_deref(), API_KEY_VAR, proxy_url).await {
        Ok(response) => println!("OpenAI Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}