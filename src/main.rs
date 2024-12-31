use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;

use std::env::args;
use std::io::{stdin, stdout, Write};

fn read_input(message: &str) -> String {
    print!("{}", message);
    let _ = stdout().flush();
	let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Did not enter a valid string");
    if let Some('\n')=user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r')=user_input.chars().next_back() {
        user_input.pop();
    }
    return user_input;
}

#[tokio::main]
async fn main() -> Result<()> {
	// Read program arguments or user input
    let key = if args().len() > 1 {
            args().nth(1).unwrap()
        } else {
            read_input("Please enter API key: ")
        };
    let prompt = if args().len() > 2 {
            args().nth(2).unwrap()
        } else {
	        read_input("Please enter a prompt: ")
        };

	// Create new API client
    let client = ChatGPT::new(key) ?;

    // Send message
    let response: CompletionResponse = client.send_message(prompt).await ?;

    println!("Response: {}", response.message().content);

    Ok(())
}
