use crate::dom;
use crate::utils;
use crate::RedirectConfig;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn perform_redirect(url: &str) {
    if !utils::validate_url(url) {
        utils::log(&format!("Invalid URL: {}", url));
        return;
    }

    let window = web_sys::window().expect("should have a window");

    match window.location().set_href(url) {
        Ok(_) => utils::log(&format!("Redirecting to {}", url)),
        Err(e) => utils::log(&format!("Redirect failed: {:?}", e)),
    }
}

pub fn start_countdown(config: RedirectConfig) {
    let target_url = config.target_url();
    let delay = config.delay_seconds();
    let show_countdown = config.show_countdown();

    let remaining = Rc::new(RefCell::new(delay));
    let cancelled = Rc::new(RefCell::new(false));
    let window = web_sys::window().expect("should have a window");

    let countdown_closure = Rc::new(RefCell::new(None));
    let countdown_closure_clone = countdown_closure.clone();

    let cancelled_clone = cancelled.clone();
    let interval_callback = Closure::wrap(Box::new(move || {
        // Check if cancelled
        if *cancelled_clone.borrow() {
            if let Some(handle) = countdown_closure_clone.borrow_mut().take() {
                let window = web_sys::window().expect("should have a window");
                window.clear_interval_with_handle(handle);
            }
            return;
        }

        let mut remaining_seconds = remaining.borrow_mut();

        if *remaining_seconds == 0 {
            if let Some(handle) = countdown_closure_clone.borrow_mut().take() {
                let window = web_sys::window().expect("should have a window");
                window.clear_interval_with_handle(handle);
            }

            perform_redirect(&target_url);
        } else {
            *remaining_seconds -= 1;

            if show_countdown {
                if let Err(e) = dom::update_countdown(*remaining_seconds) {
                    utils::log(&format!("Error updating countdown: {:?}", e));
                }
            }
        }
    }) as Box<dyn FnMut()>);

    if show_countdown {
        if let Err(e) = dom::update_countdown(delay) {
            utils::log(&format!("Error setting initial countdown: {:?}", e));
        }
    }

    match window.set_interval_with_callback_and_timeout_and_arguments_0(
        interval_callback.as_ref().unchecked_ref(),
        1000,
    ) {
        Ok(handle) => {
            *countdown_closure.borrow_mut() = Some(handle);

            // Set up cancel button with access to the cancelled flag
            if config.allow_cancel() {
                if let Err(e) = dom::setup_cancel_button(cancelled) {
                    utils::log(&format!("Error setting up cancel button: {:?}", e));
                }
            }

            interval_callback.forget();
        }
        Err(e) => {
            utils::log(&format!("Failed to start countdown: {:?}", e));
        }
    }
}
