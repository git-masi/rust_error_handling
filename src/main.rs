use dialoguer::{theme::ColorfulTheme, Select};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let selections = [
        "no error handling - print text",
        "no error handling - panic",
        "handle TCP errors - print DNS error",
        "handle TCP errors - print TCP error",
        "no error handling - print text despite 404 response",
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

/// This will panic
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
