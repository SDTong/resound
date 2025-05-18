//! process operation

use std::borrow::Cow;

use audio::process;

use crate::interactive::{print_list, PROMPT_DEFAULT_COW, PROMPT_ERR_COMMOND_COW};

pub(super) fn run_commond<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let token = commond_iter.next();
    match token {
        Some("listall") => list_all(),
        _ => PROMPT_ERR_COMMOND_COW,
    }
}

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
