//! 录系统播放的声音
//!
//! 交互式命令，通常情况下，格式为： 对象 + 动作 + 参数

mod interactive;
mod operate;

fn main() {
    operate::run();
}
