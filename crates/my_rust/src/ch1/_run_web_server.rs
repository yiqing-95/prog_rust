// Import necessary crates and modules
use actix_web::{web, App, HttpResponse, HttpServer};
use futures::future::join;
use tokio::runtime::Runtime;

// Define a handler function for the web server
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

// Define a function to run the Actix Web server with runtime::Runtime and block_on
fn run_server() {
    // Create a new runtime
    let rt = Runtime::new().unwrap();

    // Spawn a new thread to run the Actix Web server
    let server = rt.spawn(async {
        // Start the Actix Web server
        HttpServer::new(|| {
            App::new().service(web::resource("/").to(index))
        })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();
    });

    // Block the current thread and run both the server and the runtime concurrently
    rt.block_on(async {
        // Use the `join` function from the `futures` crate to run both the server and the runtime concurrently
        join(server, async {
            // Perform any other tasks here if needed
            // This block will run concurrently with the server

            // For example, let's print a message
            println!("Other tasks running concurrently with the server");

            // Sleep for 5 seconds to simulate some other tasks
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            // Print a message after the sleep
            println!("Other tasks completed");
        })
        .await;
    });
}

fn main() {
    // Call the function to run the server
    run_server();
}