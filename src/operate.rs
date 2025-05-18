//! analyse commond, run operate

use std::borrow::Cow;

use crate::interactive::PROMPT_ERR_COMMOND_COW;

mod process_ope;

// start analyse and run commond
pub(super) fn run_commond<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let token = commond_iter.next();
    match token {
        Some("process") => process_ope::run_commond(commond_iter),
        _ => PROMPT_ERR_COMMOND_COW,
    }
}
