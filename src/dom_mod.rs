//! dom_mod.rs manipulates the html dom

use crate::web_sys_mod::*;

/// The app starts with this function
pub fn start_function() {
    // inject html into DOM
    inject_htm_into_dom();
    // prepare events that read local file, pass the function to execute
    add_listener_to_button("button_1", &on_click_button_1);
}

/// inject html into dom
pub fn inject_htm_into_dom() {
    let html = format!(
        r##"
<h2>Sort text</h2>

<textarea rows="20" cols="50" id="my_text" ></textarea>

<div>
    <p>Enter a locale. It can be: en, de, sl, hr,...</p>
    <input id="locale" style="width:50px" type="text" value="sl"/>
    <input id="button_1" type="button" class="button" value="Sort"/>
</div>
        "##
    );

    set_inner_html("div_for_wasm_html_injecting", &html);
}

/// the listener calls this function
fn on_click_button_1() {
    let locale = get_input_element_value_string_by_id("locale");
    let my_text = get_text_area_element_value_string_by_id("my_text");
    let list: Vec<&str> = my_text.lines().collect();

    // the special sorting with crate feruca
    let list = crate::sorting_mod::sort(list, &locale);

    let my_new_text = list.join("\n");
    set_text_area_element_value_string_by_id("my_text", &my_new_text);
}
