//! sorting_mod.rs module for sorting and collation

use crate::web_sys_mod::*;

pub fn sort<'a>(mut list: Vec<&'a str>) -> Vec<&'a str> {
    // Rust does not have stable international sorting
    // I will use the javascript Intl Collator

    let collator = create_collator("sl");

    list.sort_by(|a, b| collator_compare(&collator, a, b));
    //return
    list
}
