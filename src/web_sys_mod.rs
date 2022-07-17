//! web_sys_mod.rs
//! helper functions for web_sys, window, dom, console, html elements,...
//! Trying to isolate/hide all javascript code and conversion in this module.

// region: use
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}
/*
/// HTML encode - naive
pub fn html_encode(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
*/

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// set inner html into dom
/// The inner_html must be correctly HTML encoded !
pub fn set_inner_html(element_id: &str, inner_html: &str) {
    let div_for_wasm_html_injecting = get_element_by_id(element_id);
    div_for_wasm_html_injecting.set_inner_html(inner_html);
}

/// add event listener for button
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// get text_area element value string by id
pub fn get_text_area_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let text_area_html_element =
        unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    // debug_write("before value()");
    text_area_html_element.value()
}

/// set text_area element value string by id
pub fn set_text_area_element_value_string_by_id(element_id: &str, value: &str) {
    //debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    //debug_write("before dyn_into");
    let text_area_html_element =
        unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    //debug_write("before value()");
    text_area_html_element.set_value(value);
}

pub fn create_collator(locale: &str) -> js_sys::Intl::Collator {
    let array = js_sys::Array::new();
    debug_write(locale);
    let locale_js = wasm_bindgen::JsValue::from_str(locale);
    array.push(&locale_js);
    let object = js_sys::Object::new();
    let collator = js_sys::Intl::Collator::new(&array, &object);
    // return
    collator
}

pub fn collator_compare(collator: &js_sys::Intl::Collator, a: &str, b: &str) -> std::cmp::Ordering {
    // console.log(['Z', 'a', 'z', 'ä'].sort(new Intl.Collator('de').compare));
    match collator
        .compare()
        .call2(&JsValue::NULL, &JsValue::from_str(a), &JsValue::from_str(b))
    {
        Err(_err) => std::cmp::Ordering::Equal,
        Ok(js_value) => {
            if js_value.as_f64().unwrap() > 0.0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }
    }
}
