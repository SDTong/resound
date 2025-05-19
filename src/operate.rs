//! analyse commond, run operate

use std::borrow::Cow;

use crate::interactive::{self, PROMPT_DEFAULT_COW, PROMPT_ERR_COMMOND_COW};

mod process_ope;

// start
pub(super) fn run() {
    let mut commond;
    let mut prompt = PROMPT_DEFAULT_COW;
    let mut commond_iter;
    loop {
        commond = interactive::wait_commond(&prompt);
        commond_iter = commond.split_whitespace();
        let token = commond_iter.next();
        prompt = match token {
            Some("help") => help(),
            Some("quit") => break,
            Some("process") => process_ope::run_commond(&mut commond_iter),
            _ => PROMPT_ERR_COMMOND_COW,
        };
    }
    
}

fn help() -> Cow<'static, str> {
    interactive::print_line("if you want to view details for command, please use \"Command help\" ");
    interactive::print_list(&HELP_CONTENT);
    PROMPT_DEFAULT_COW
}

const HELP_CONTENT: [[(Cow<'_, str>, Cow<'_, str>); 1]; 1] = [[(
    Cow::Borrowed("process"),
    Cow::Borrowed("I think process is one with suppoer audio"),
)]];
