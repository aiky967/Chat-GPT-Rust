use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env; // env module for env variables, OpenAi access key
use std::io::{stdin, stdout, Write};

// a struct for the choices
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// a struct to wrok with API response
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

// a struct for the request to make to the API
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // load the env variables
    dotenv().ok();
    // create a Httpsconnector, hyper
    let https = HttpsConnector::new();
    // create a client
    let client = Client::builder().build(https);
    // URL to which we will make the request
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    // preamble, prompt to chatGPT -> You can use this prompt to get the answers in your interests
    // let preamble = "Answer the following question
    // accurately, but find a funny way to mention
    // the Rust programming language in your response.";
    let preamble = "Write a SQL query according to the sentence";
    // token, in the header read from .env file
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop {
        // loop, inside the loop a way to read user input
        print!(">");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
        println!("");

        // spinner, wait for the response
        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into());
        // request to chatGPT for every singleuser input, loop
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 1000,
        };

        // request the api using uri
        let body = Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        // response and we print the response
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json: OAIResponse = serde_json::from_reader(body.reader())?;
        sp.stop();
        println!("");
        println!("{}", json.choices[0].text);

    }

    Ok(())

}


