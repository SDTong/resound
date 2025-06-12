//! user interactive

use std::borrow::Cow;

// promtp start
// default
const PROMPT_DEFAULT: &str = "please input command(help can show all command): ";
pub(crate) const PROMPT_DEFAULT_COW: Cow<'_, str> = Cow::Borrowed(PROMPT_DEFAULT);

// no command
const PROMPT_ERR_COMMAND: &str = "unsupported command";
pub(crate) const PROMPT_ERR_COMMAND_COW: Cow<'_, str> = Cow::Borrowed(PROMPT_ERR_COMMAND);
// promtp end

// show one line
pub(super) fn print_line(data: &str) {
    println!("\n{}", data);
}

// show list info
pub(crate) fn print_list<'a, I, J>(data: I)
where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = &'a (Cow<'a, str>, Cow<'a, str>)>,
{
    println!();
    data.into_iter().for_each(|line_data| {
        let mut line_data = line_data.into_iter();
        let first = line_data.next();
        if let None = first {
            // no data
            return;
        }
        let first = first.unwrap();
        print!("{}: {}", first.0, first.1);
        line_data.for_each(|one_node| print!("; {}: {}", one_node.0, one_node.1));
        println!();
    });
}
