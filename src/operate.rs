//! analyse commond, run operate

use std::borrow::Cow;

use crate::interactive::{self, PROMPT_ERR_COMMOND_COW};

mod process_ope;

// start analyse and run commond
pub(super) fn run_commond<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let token = commond_iter.next();
    match token {
        Some("process") => process_ope::run_commond(commond_iter),
        Some("help") => help(),
        _ => PROMPT_ERR_COMMOND_COW,
    }
}

fn help() -> Cow<'static, str> {
    interactive::print_line("if you want to view details for command, please use \"Command help\" ");
    interactive::print_list(&HELP_CONTENT);
    PROMPT_ERR_COMMOND_COW
}

const HELP_CONTENT: [[(Cow<'_, str>, Cow<'_, str>); 1]; 1] = [[(
    Cow::Borrowed("process"),
    Cow::Borrowed("I think process is one with suppoer audio"),
)]];
