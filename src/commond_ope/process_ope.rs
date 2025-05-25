//! process commond operation

use std::borrow::Cow;

use audio::process;

use crate::interactive::{PROMPT_DEFAULT_COW, PROMPT_ERR_COMMOND_COW, print_list};

pub(super) fn run_commond<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let token = commond_iter.next();
    match token {
        Some("help") => help(),
        Some("listall") => list_all(),
        _ => PROMPT_ERR_COMMOND_COW,
    }
}

// show help
fn help() -> Cow<'static, str> {
    print_list(&HELP_CONTENT);
    PROMPT_DEFAULT_COW
}

const HELP_CONTENT: [[(Cow<'_, str>, Cow<'_, str>); 1]; 2] = [
    [(Cow::Borrowed("help"), Cow::Borrowed("show this"))],
    [(Cow::Borrowed("listall"), Cow::Borrowed("show all process"))],
];

// show all process
fn list_all() -> Cow<'static, str> {
    let process_vec = process::list().unwrap();
    let content_vec = process_vec
        .iter()
        .map(|process| {
            let bundle_id = process
                .get_bundle_id()
                .map(|bundle_id| Cow::from(bundle_id))
                .unwrap_or(Cow::from("query err"));
            let vec = vec![
                (Cow::from("id"), Cow::from(process.get_id().to_string())),
                (Cow::from("budle id"), bundle_id),
            ];
            vec
        })
        .collect::<Vec<Vec<(Cow<'_, str>, Cow<'_, str>)>>>();
    print_list(&content_vec);

    PROMPT_DEFAULT_COW
}
