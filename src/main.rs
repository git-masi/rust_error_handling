use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Select};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();
    let selections = [
        "no error handling - print text",
        "no error handling - panic",
        "handle TCP errors - print DNS error",
        "handle TCP errors - print TCP error",
        "no HTTP status error handling - print text despite 404 response",
        "handle HTTP status >= 400 - print 404 error",
        "no JSON parse error handling - panic",
        "handle JSON parse error handling - print JSON parse error",
        "handle error message in JSON response body - print error in response body",
        "handle all errors using a terser interface",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an example to execute.")
        .items(&selections[..])
        .default(0)
        .interact()
        .unwrap();

    if selection == 0 {
        example_one(client.clone()).await;
    } else if selection == 1 {
        example_two(client.clone()).await;
    } else if selection == 2 {
        example_three(client.clone()).await;
    } else if selection == 3 {
        example_four(client.clone()).await;
    } else if selection == 4 {
        example_five(client.clone()).await;
    } else if selection == 5 {
        example_six(client.clone()).await;
    } else if selection == 6 {
        example_seven(client.clone()).await;
    } else if selection == 7 {
        example_eight(client.clone()).await;
    } else if selection == 8 {
        example_nine(client.clone()).await;
    } else if selection == 9 {
        example_ten(client.clone()).await;
    }
}

async fn example_one(client: reqwest::Client) {
    let response = client
        .get("https://example.com")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{response}");
}

/// This will panic, DNS error
async fn example_two(client: reqwest::Client) {
    let response = client
        .get("https://fake.fake.fake")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{response}");
}

/// This does not panic, prints DNS error
async fn example_three(client: reqwest::Client) {
    match client.get("https://fake.fake.fake").send().await {
        Ok(response) => {
            println!("{}", response.text().await.unwrap());
        }
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    };
}

/// This does not panic, prints TCP error
async fn example_four(client: reqwest::Client) {
    match client.get("http://localhost:5555").send().await {
        Ok(response) => {
            println!("{}", response.text().await.unwrap());
        }
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    };
}

/// This response is 404 but it does not panic
async fn example_five(client: reqwest::Client) {
    let response = client
        .get("https://wikipedia.com/not/a/real/path")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{response}");
}

/// Handle 404 response
async fn example_six(client: reqwest::Client) {
    match client
        .get("https://wikipedia.com/not/a/real/path")
        .send()
        .await
    {
        Ok(response) => match response.error_for_status() {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(e) => {
                eprintln!("Received HTTP error status in response:\n{e}");
            }
        },
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    }
}

/// This will panic, can't parse JSON
async fn example_seven(client: reqwest::Client) {
    match client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await
    {
        Ok(response) => match response.error_for_status() {
            Ok(response) => {
                println!(
                    "{:?}",
                    response.json::<fail_to_parse::Todo>().await.unwrap()
                );
            }
            Err(e) => {
                eprintln!("Received HTTP error status in response:\n{e}");
            }
        },
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    }
}

/// This will print an error
async fn example_eight(client: reqwest::Client) {
    match client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await
    {
        Ok(response) => match response.error_for_status() {
            Ok(response) => match response.json::<fail_to_parse::Todo>().await {
                Ok(json) => {
                    println!("{:?}", json);
                }
                Err(e) => {
                    eprintln!("Error parsing JSON response:\n{e}");
                }
            },
            Err(e) => {
                eprintln!("Received HTTP error status in response:\n{e}");
            }
        },
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    }
}

mod fail_to_parse {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Todo {
        pub cannot_parse: bool,
    }
}

/// This prints an error
async fn example_nine(client: reqwest::Client) {
    let body = error_in_response::RpcRequest::new();

    match client
        .post("http://localhost:8080/rpc")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => match response.error_for_status() {
            Ok(response) => match response.json::<error_in_response::RpcResponse>().await {
                Ok(json) => {
                    if let Some(error) = json.error {
                        eprintln!("Error from RPC server:\n{}", error.message);
                    } else if let Some(result) = json.result {
                        println!("{}", result);
                    } else {
                        eprintln!("Expected a response or an error and got:\n{:?}", json);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing JSON response:\n{e}");
                }
            },
            Err(e) => {
                eprintln!("Received HTTP error status in response:\n{e}");
            }
        },
        Err(e) => {
            eprintln!("Error making HTTP request:\n{e}");
        }
    }
}

mod error_in_response {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct RpcRequest {
        pub method: &'static str,
        pub params: (&'static str, &'static str),
    }

    impl RpcRequest {
        pub fn new() -> Self {
            Self {
                method: "unknown",
                params: ("123abc", "987xyz"),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct RpcResponse {
        pub result: Option<String>,
        pub error: Option<RpcResponseError>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RpcResponseError {
        pub message: String,
    }
}

/// This prints an error
async fn example_ten(client: reqwest::Client) {
    let body = nicer_error_handling::RpcRequest::new();

    let builder = client.post("http://localhost:8080/rpc").json(&body);

    match nicer_error_handling::handle_request::<String>(builder).await {
        Ok(result) => {
            println!("{result}");
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

mod nicer_error_handling {
    use super::*;

    #[derive(thiserror::Error, Debug)]
    pub enum AppError {
        #[error("Unexpected TCP error:\n{0}")]
        Tcp(reqwest::Error),
        #[error("Request came back with HTTP error response code:\n{0}")]
        HttpResponse(reqwest::Error),
        #[error("Could not parse JSON response:\n{0}")]
        JsonParse(reqwest::Error),
        #[error("Error message in JSON response:\n{0}")]
        Message(String),
        #[error("An unexpected error occurred:\n{0}")]
        Unexpected(String),
    }

    pub async fn handle_request<T: DeserializeOwned>(
        builder: reqwest::RequestBuilder,
    ) -> Result<T, AppError> {
        let json = builder
            .send()
            .await
            .map_err(|e| AppError::Tcp(e))?
            .error_for_status()
            .map_err(|e| AppError::HttpResponse(e))?
            .json::<RpcResponse<T>>()
            .await
            .map_err(|e| AppError::JsonParse(e))?;

        if let Some(error) = json.error {
            Err(AppError::Message(error.message))
        } else if let Some(result) = json.result {
            Ok(result)
        } else {
            Err(AppError::Unexpected(format!(
                "Expected either a result or an error in the RPC response object and found neither"
            )))
        }
    }

    #[derive(Debug, Serialize)]
    pub struct RpcRequest {
        pub method: &'static str,
        pub params: (&'static str, &'static str),
    }

    impl RpcRequest {
        pub fn new() -> Self {
            Self {
                method: "unknown",
                params: ("123abc", "987xyz"),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct RpcResponse<T> {
        pub result: Option<T>,
        pub error: Option<RpcResponseError>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RpcResponseError {
        pub message: String,
    }
}
