//! analyse commond, run operate

use std::borrow::Cow;

use crate::interactive::{self, PROMPT_DEFAULT_COW, PROMPT_ERR_COMMOND_COW};

mod process_ope;
mod re_ope;

const TAP_NAME_DEFAULT: &str = "resoundTap";

// start
pub(super) fn run() {
    let mut commond;
    let mut prompt = PROMPT_DEFAULT_COW;
    let mut commond_iter;
    loop {
        commond = interactive::wait_commond(&prompt);
        commond_iter = commond.split_whitespace();
        let commond_iter = &mut commond_iter;
        let token = commond_iter.next();
        prompt = match token {
            Some("help") => help(),
            // 友好的退出
            // todo 监听 ctrl + c、kill等，在退出时执行相同的处理
            // todo 关闭正在执行的录音对象，清理tap等内容
            // 测试kill、ctrl + c 、painc 等场景下，结构体的drop方法是否会执行
            // painc 回执行drop方法，其它场景不会
            Some("quit") => break,
            Some("process") => process_ope::run_commond(commond_iter),
            // 录音相关
            Some("re") => re_ope::run_commond(commond_iter),
            _ => PROMPT_ERR_COMMOND_COW,
        };
    }
}

fn help() -> Cow<'static, str> {
    interactive::print_line(
        "if you want to view details for command, please use \"Command help\" ",
    );
    interactive::print_list(&HELP_CONTENT);
    PROMPT_DEFAULT_COW
}

const HELP_CONTENT: [[(Cow<'_, str>, Cow<'_, str>); 1]; 4] = [
    [(Cow::Borrowed("help"), Cow::Borrowed("show this"))],
    [(Cow::Borrowed("quit"), Cow::Borrowed("quit resound"))],
    [(
        Cow::Borrowed("process"),
        Cow::Borrowed("I think process is one with suppoer audio"),
    )],
    [(Cow::Borrowed("re"), Cow::Borrowed("record sound"))],
];
