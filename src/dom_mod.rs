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
        <h2 id="hello_text">Hello World !</h2>
		
        <div class="button-wrap">
            <label for="my_name">Enter your name:</label>  
            <input style="width:40%;" type="text" id="my_name" value="my_name"/>
        </div>

        <!--tricky div+label+css to change Input file appearance -->
        <div class="button-wrap">
            <input id="button_1" type="button" class="button" value="Reload"/>
        </div>
        "##
    );

    set_inner_html("div_for_wasm_html_injecting", &html);
}

/// the listener calls this function
fn on_click_button_1() {
    let my_name = get_input_element_value_string_by_id("my_name");
    let my_name = html_encode(&my_name);
    set_inner_html("hello_text", &format!("Hello {my_name} !"));
}
