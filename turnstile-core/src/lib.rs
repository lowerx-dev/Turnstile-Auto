use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, ShadowRoot};

static mut INTERVAL_ID: Option<i32> = None; // เก็บ interval_id

fn console_log(msg: &str) {
    web_sys::console::log_1(&format!("[🤖] [ Turnstile Bot ] - {}", msg).into());
}

fn clear_loop() {
    unsafe {
        if let Some(interval_id) = INTERVAL_ID {
            window().unwrap().clear_interval_with_handle(interval_id);
            INTERVAL_ID = None;
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn getOpenOrClosedShadowRoot(element: &Element) -> Option<ShadowRoot>;
}

#[wasm_bindgen]
pub fn inject() {
    let window = window().unwrap();
    let location = window.location().href().unwrap();

    if location.contains("challenges.cloudflare.com/cdn-cgi/challenge-platform/") {
        console_log("Detected Cloudflare CAPTCHA ✅");

        let closure = Closure::wrap(Box::new(move || {
            let document = web_sys::window().unwrap().document().unwrap();
            let success_element = document.get_element_by_id("success");

            let is_visible = success_element
                .as_ref()
                .map(|e| {
                    let html_element = e.dyn_ref::<HtmlElement>().unwrap();
                    html_element.style().get_property_value("visibility").unwrap() == "visible"
                })
                .unwrap_or(false);

            if !is_visible {
                let shadow_body = getOpenOrClosedShadowRoot(&document.body().unwrap().dyn_into::<Element>().unwrap().into());

                if let Some(shadow_body) = shadow_body {
                    let shadow_document = shadow_body.dyn_into::<ShadowRoot>();

                    match shadow_document {
                        Ok(shadow_document) => {
                            let checkboxes = shadow_document
                                .query_selector("#content > div > div > label")
                                .unwrap();

                            if let Some(checkbox) = checkboxes {
                                checkbox.dyn_ref::<HtmlElement>().unwrap().click();
                                console_log("Clicked checkbox ✅");

                                // เมื่อกด checkbox แล้ว หยุด interval
                                clear_loop();
                            }
                            return;
                        },
                        Err(_) => {}
                    }
                }
            }
        }) as Box<dyn Fn()>);

        let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 500).unwrap();
        
        // เก็บ interval_id
        unsafe {
            INTERVAL_ID = Some(interval_id);
        }

        closure.forget();
    }
}