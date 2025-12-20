//! WMS Frontend Entry Point

use leptos::*;
use wms_frontend::App;

fn main() {
    // Initialize console logging for debugging
    console_error_panic_hook::set_once();
    
    // Mount the app to the document body
    mount_to_body(|| view! { <App/> });
}

