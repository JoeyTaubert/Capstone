use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension, Router,
};
use axum_macros::debug_handler;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
mod analysis;
mod cap;

// Reference: https://github.com/programatik29/axum-tutorial/blob/master/tutorial/01-introduction.md
// Axum docs: https://docs.rs/axum/latest/axum/#example

// STRUCTS -=-=-=-=-=-=-=-=-=-=-=-=

/// Context struct for interfaces dropdown on capture_template.html  
#[derive(Serialize)]
struct InterfacesContext {
    interfaces: Vec<String>,
}

#[derive(Serialize)]
struct CaptureContext {
    is_running: String,
    interface: String,
    num_packets: u32,
}

#[derive(Clone, Deserialize, Default)]
/// Struct for holding the capture parameters
/// Could manually impl default for default values
struct CaptureParams {
    interface: String,
    num_packets: u32,
}

// State tracking of the capture parameters for shared state
#[derive(Clone, Default)]
struct CaptureConfig {
    capture_params: Arc<RwLock<CaptureParams>>, // Mutex/Rwlock for preventing multiple threads from writing to the data at once
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
async fn capture_page(
    State(state): State<CaptureConfig>,
    Extension(handlebars): Extension<Arc<Handlebars<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let params = state.capture_params.read().await;

    let interface = params.interface.clone();
    let num_packets = params.num_packets;

    let is_running = String::from("N/A");

    let context = CaptureContext {
        is_running,
        interface,
        num_packets,
    };

    let rendered = handlebars
        .render("capture_template", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
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

/// Handler for packet number form submission
///
async fn submit_capture(
    State(state): State<CaptureConfig>,
    Form(data): Form<CaptureParams>,
) -> Redirect {
    let mut params = state.capture_params.write().await;
    params.interface = data.interface;
    params.num_packets = data.num_packets;
    Redirect::to("/capture.html")
}

async fn init_capture(State(state): State<CaptureConfig>) -> impl IntoResponse {
    // Initialize the capture parameters
    let params = state.capture_params.read().await;

    let interface = params.interface.clone();
    let num_packets = params.num_packets;

    // May need to change this if capture consists of blocking I/O work
    // Spawn a new concurrent task
    tokio::spawn(async move { cap::start_capture(interface, num_packets).await });

    Redirect::to("/capture.html")
}

//async fn restart_capture() {}

//async fn stop_capture() {}

async fn analysis_page() -> Html<String> {
    match tokio::fs::read_to_string("static/html/analysis.html").await {
        Ok(html_content) => Html(html_content),
        Err(e) => {
            println!("{}", e);
            Html("Error loading the page".to_string())
        }
    }
}

async fn submit_analysis() {
    // state logic
    
    // Logic to check selected metric and call proper compute function

    
}

#[tokio::main]
async fn main() {
    // Initialize Handlebars
    let mut handlebars = Handlebars::new();

    // Get path of template HTMLs
    let edit_capture_path = PathBuf::from("static/html/capture/edit_capture_template.hbs");
    let capture_path = PathBuf::from("static/html/capture.hbs");

    // Register the templates with Handlebars
    handlebars
        .register_template_file("edit_capture_template", edit_capture_path)
        .expect("Failed to register template");

    handlebars
        .register_template_file("capture_template", capture_path)
        .expect("Failed to register template");

    // Wraps the handlebars instance in the "Atomic Reference Counter" type, used to safely share across multiple threads
    let handlebars = Arc::new(handlebars);

    // Shared state tracking for capture parameters
    let capture_config = CaptureConfig {
        capture_params: Arc::new(RwLock::new(CaptureParams::default())),
    };

    // Define app routes
    let app = Router::new()
        .route("/", get(index_page))
        .route("/capture.html", get(capture_page))
        .route("/capture/edit.html", get(capture_edit_settings))
        .route("/capture/start.html", get(init_capture))
        .route("/capture/submit", post(submit_capture))
        .route("/analysis.html", get(analysis_page))
        .route("/analysis/submit", post(submit_analysis))
        .layer(Extension(capture_config.clone()))
        .layer(Extension(handlebars))
        .with_state(capture_config.clone());

    // Run app, listening on loopback only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
