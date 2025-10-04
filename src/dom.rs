use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlButtonElement, HtmlElement};

pub fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    window.document().ok_or("No document".into())
}

pub fn update_message(message: &str) -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(element) = document.get_element_by_id("message") {
        element.set_text_content(Some(message));
    }

    Ok(())
}

pub fn update_countdown(seconds: u32) -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(element) = document.get_element_by_id("countdown") {
        let text = if seconds == 1 {
            format!("{} second remaining...", seconds)
        } else {
            format!("{} seconds remaining...", seconds)
        };
        element.set_text_content(Some(&text));
    }

    Ok(())
}

pub fn hide_element(id: &str) -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(element) = document.get_element_by_id(id) {
        let html_element = element.dyn_into::<HtmlElement>()?;
        let style = html_element.style();
        style.set_property("display", "none")?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn show_element(id: &str) -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(element) = document.get_element_by_id(id) {
        let html_element = element.dyn_into::<HtmlElement>()?;
        let style = html_element.style();
        style.set_property("display", "block")?;
    }

    Ok(())
}

pub fn setup_cancel_button() -> Result<(), JsValue> {
    let document = get_document()?;

    if let Some(button) = document.get_element_by_id("cancel-button") {
        let html_button = button.dyn_into::<HtmlButtonElement>()?;
        let style = html_button.style();
        style.set_property("display", "inline-block")?;

        let closure = Closure::wrap(Box::new(move || {
            crate::utils::log("Redirect cancelled by user");
            let _ = update_message("Redirect cancelled");
            let _ = hide_element("countdown");
            let _ = hide_element("cancel-button");
        }) as Box<dyn FnMut()>);

        html_button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}
