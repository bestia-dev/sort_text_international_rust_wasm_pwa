//! sorting_mod.rs module for sorting and collation

use crate::web_sys_mod::*;

pub fn sort<'a>(mut list: Vec<&'a str>, locale: &str) -> Vec<&'a str> {
    let collator = create_collator(locale);

    list.sort_by(|a, b| collator_compare(&collator, a, b));
    //return
    list
}
