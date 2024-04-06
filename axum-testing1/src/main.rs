use axum::{
    extract::Form,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Extension, Json, Router,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::fs::read_to_string;
mod cap;
mod analysis;

// Reference: https://github.com/programatik29/axum-tutorial/blob/master/tutorial/01-introduction.md
// Axum docs: https://docs.rs/axum/latest/axum/#example

// STRUCTS -=-=-=-=-=-=-=-=-=-=-=-=

/// Context struct for interfaces dropdown on capture_template.html  
#[derive(Serialize)]
struct InterfacesContext {
    interfaces: Vec<String>,
}

#[derive(Deserialize)]
struct InterfaceFormData {
    interface: String,
}

#[derive(Deserialize)]
struct PacketFormData {
    num_packets: i32,
}

// FUNCTIONS -=-=-=-=-=-=-=-=-=-=-=-=

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

// HANDLERS -=-=-=-=-=-=-=-=-=-=-=-=

/// Handler to serve the root route "/"
async fn index_page() -> Html<String> {
    match tokio::fs::read_to_string("static/html/index.html").await {
        Ok(html_content) => Html(html_content),
        Err(e) => {
            println!("{}", e);
            Html("Error loading the page".to_string())
        }
    }
}

/// Handler to serve capture.html
async fn capture_page() -> Html<String> {
    match tokio::fs::read_to_string("static/html/capture.html").await {
        Ok(html_content) => Html(html_content),
        Err(e) => {
            println!("{}", e);
            Html("Error loading the page".to_string())
        }
    }
}

/// Handler for /capture/edit.html route
async fn capture_edit_settings(
    Extension(handlebars): Extension<Arc<Handlebars<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let interfaces = interface_list();

    let context = InterfacesContext { interfaces };

    let rendered = handlebars
        .render("edit_capture_template", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}

/// Handler for interface form submission
///
async fn submit_capture_interface(
    Form(data): Form<InterfaceFormData>,
) -> (StatusCode, Json<&'static str>) {
    // Parse out data from the submission
    println!("Selected interface: {}", data.interface);

    // Return status code
    (StatusCode::OK, Json("Interface submitted successfully"))
}

/// Handler for packet number form submission
///
async fn submit_capture_packets(
    Form(data): Form<PacketFormData>,
) -> (StatusCode, Json<&'static str>) {
    // Parse out data
    println!("Number of packets to capture: {}", data.num_packets);

    // Return status code OK, with string
    (StatusCode::OK, Json("Packets submitted successfully"))
}

async fn start_capture() {
    
}

//async fn restart_capture() {}

//async fn stop_capture() {}

#[tokio::main]
async fn main() {
    // Initialize Handlebars
    let mut handlebars = Handlebars::new();

    // Get contents of template HTMLs
    let edit_capture_path = PathBuf::from("static/html/capture/edit_capture_template.html");
    let edit_capture_template_content = read_to_string(edit_capture_path)
        .await
        .expect("Cannot read template file");

    // Register the templates with Handlebars
    handlebars
        .register_template_string("edit_capture_template", &edit_capture_template_content)
        .expect("Failed to register template");

    // Wraps the handlebars instance in the "Atomic Reference Counter" type, used to safely share across multiple threads
    let handlebars = Arc::new(handlebars);

    // Define app routes
    let app = Router::new()
        .route("/", get(index_page))
        .route("/capture.html", get(capture_page))
        .route("/capture/edit.html", get(capture_edit_settings))
        .route("/capture/start.html", get(start_capture))
        //.route("/capture/restart.html", get(restart_capture))
        //.route("/capture/stop.html", get(stop_capture))
        .route("/capture/submit-interface", post(submit_capture_interface))
        .route("/capture/submit-packets", post(submit_capture_packets))
        .layer(Extension(handlebars));

    // Run app, listening on loopback only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
