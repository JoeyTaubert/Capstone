use axum::{http::StatusCode, response::Html, routing::get, Extension, Router};
use handlebars::Handlebars;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::fs::read_to_string;

// Reference: https://github.com/programatik29/axum-tutorial/blob/master/tutorial/01-introduction.md
// Axum docs: https://docs.rs/axum/latest/axum/#example

/// Handler to serve the root route "/"
async fn index_page() -> Html<String> {
    match tokio::fs::read_to_string("static/html/index.html").await {
        Ok(html_content) => Html(html_content),
        Err(e) => {
            println!("{}", e);
            Html("Error loading the form".to_string())
        }
    }
}

/// Context struct for content handlebars will use to fill in the capture_template.html  
#[derive(Serialize)]
struct Context {
    interfaces: Vec<String>,
}

/// Gets all interfaces on the server  
fn interface_list() -> Vec<String> {
    // Grab network interfaces
    let devices = pcap::Device::list().expect("[-]ERROR: Failed to grab network interfaces");

    // Print network interfaces
    let mut interfaces_vec: Vec<String> = Vec::new();

    for device in devices {
        interfaces_vec.push(device.name.clone());
    }

    interfaces_vec
}

/// Handler for /capture.html route
async fn capture_page_interfaces(
    Extension(handlebars): Extension<Arc<Handlebars<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let interfaces = interface_list();

    let context = Context { interfaces };

    let rendered = handlebars
        .render("capture_template", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}

#[tokio::main]
async fn main() {
    // Initialize Handlebars
    let mut handlebars = Handlebars::new();

    // Get contents of template HTML
    let path = PathBuf::from("static/html/capture_template.html");
    let template_content = read_to_string(path)
        .await
        .expect("Cannot read template file");

    // Register the template with Handlebars
    handlebars
        .register_template_string("capture_template", &template_content)
        .expect("Failed to register template");

    // Wraps the handlebars instance in the "Atomic Reference Counter" type, used to safely share across multiple threads
    let handlebars = Arc::new(handlebars);

    // Define app routes
    let app = Router::new()
        .route("/", get(index_page))
        .route("/capture.html", get(capture_page_interfaces))
        .layer(Extension(handlebars));

    // Run app, listening on loopback only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
