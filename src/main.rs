//! 录系统播放的声音
//!
//! 交互式命令，通常情况下，格式为： 对象 + 动作 + 参数

mod interactive;
mod operate;

fn main() {
    let mut commond;
    let mut prompt = interactive::PROMPT_DEFAULT_COW;
    let mut commond_iter;
    loop {
        commond = interactive::wait_commond(&prompt);
        commond_iter = commond.split_whitespace();
        prompt = operate::run_commond(&mut commond_iter);
    }
}
