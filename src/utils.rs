use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn validate_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("/")
}
