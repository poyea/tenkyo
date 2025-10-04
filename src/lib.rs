mod dom;
mod redirect;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen]
pub struct RedirectConfig {
    target_url: String,
    delay_seconds: u32,
    show_countdown: bool,
    message: String,
    allow_cancel: bool,
}

#[wasm_bindgen]
impl RedirectConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(target_url: String, delay_seconds: u32) -> Self {
        Self {
            target_url,
            delay_seconds,
            show_countdown: true,
            message: String::from("Redirecting..."),
            allow_cancel: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn target_url(&self) -> String {
        self.target_url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn delay_seconds(&self) -> u32 {
        self.delay_seconds
    }

    #[wasm_bindgen(getter)]
    pub fn show_countdown(&self) -> bool {
        self.show_countdown
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn allow_cancel(&self) -> bool {
        self.allow_cancel
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_countdown(&mut self, show: bool) {
        self.show_countdown = show;
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_cancel(&mut self, allow: bool) {
        self.allow_cancel = allow;
    }
}

impl Default for RedirectConfig {
    fn default() -> Self {
        Self::new(String::from("https://poyea.me"), 3)
    }
}

impl RedirectConfig {
    pub fn get_target_url(&self) -> &str {
        &self.target_url
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    utils::log("WASM module initialized");
}

#[wasm_bindgen]
pub fn start_redirect(config: RedirectConfig) {
    utils::log(&format!(
        "Starting redirect to {} in {} seconds",
        config.get_target_url(),
        config.delay_seconds()
    ));

    // Update message
    if let Err(e) = dom::update_message(config.get_message()) {
        utils::log(&format!("Error updating message: {:?}", e));
    }

    // Set up cancel button if needed
    if config.allow_cancel() {
        if let Err(e) = dom::setup_cancel_button() {
            utils::log(&format!("Error setting up cancel button: {:?}", e));
        }
    }

    // Start countdown or immediate redirect
    if config.delay_seconds() == 0 {
        redirect::perform_redirect(config.get_target_url());
    } else {
        redirect::start_countdown(config);
    }
}

#[wasm_bindgen]
pub fn parse_url_params() -> RedirectConfig {
    let window = web_sys::window().expect("should have a window");
    let search = window.location().search().unwrap_or_default();

    if search.is_empty() {
        return RedirectConfig::default();
    }

    let url = web_sys::Url::new_with_base(&search, &window.location().href().unwrap())
        .unwrap_or_else(|_| web_sys::Url::new(&format!("http://poyea.me{}", search)).unwrap());

    let params = url.search_params();

    let target_url = params
        .get("url")
        .unwrap_or_else(|| String::from("https://poyea.me"));

    let delay_seconds = params
        .get("delay")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);

    let message = params
        .get("message")
        .unwrap_or_else(|| String::from("Redirecting..."));

    let allow_cancel = params.get("cancel").is_some();

    let mut config = RedirectConfig::new(target_url, delay_seconds);
    config.set_message(message);
    config.set_allow_cancel(allow_cancel);

    config
}

#[wasm_bindgen]
pub fn get_window() -> Window {
    web_sys::window().expect("should have a window")
}
