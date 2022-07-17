//! lib.rs is just for the wasm_bindgen_start function and to connect to all the modules.
//! and for the big doc comments
//!
// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;

mod dom_mod;
mod sorting_mod;
mod web_sys_mod;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    web_sys_mod::debug_write(&format!(
        "{} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
    dom_mod::start_function();
    // return
    Ok(())
}
