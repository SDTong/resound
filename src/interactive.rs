//! user interactive

use std::borrow::Cow;

// promtp start
// default
const PROMPT_DEFAULT: &str = "please input commond(help can show all commond): ";
pub(crate) const PROMPT_DEFAULT_COW: Cow<'_, str> = Cow::Borrowed(PROMPT_DEFAULT);

// no commond
const PROMPT_ERR_COMMOND: &str = "unsupported commond";
pub(crate) const PROMPT_ERR_COMMOND_COW: Cow<'_, str> = Cow::Borrowed(PROMPT_ERR_COMMOND);
// promtp end

// remind user input commond
pub(super) fn wait_commond(prompt: &Cow<'_, str>) -> String {
    println!("{}", prompt);
    let mut commond = String::new();
    std::io::stdin()
        .read_line(&mut commond)
        .expect("read error");

    commond
}

// show one line
pub(super) fn print_line(data: &str) {
    println!("{}", data);
}

// show list info
pub(crate) fn print_list<'a, I, J>(data: I)
where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = &'a (Cow<'a, str>, Cow<'a, str>)>,
{
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
    println!();
}
