//! analyse command, run operate

use std::borrow::Cow;

use tokio::sync::{mpsc, oneshot};

use crate::interactive::{self, PROMPT_DEFAULT_COW, PROMPT_ERR_COMMAND_COW};

mod process;
mod re;

const TAP_NAME_DEFAULT: &str = "resoundTap";

pub(super) fn wait_command(tx: mpsc::Sender<(String, oneshot::Sender<()>)>) {
    loop {
         unsafe {
            // 调用 termios.h 中的 tcflush 函数。
            // libc::STDIN_FILENO 对应 C 的 STDIN_FILENO (通常是 0)。
            // libc::TCIFLUSH 告诉函数清空输入缓冲区。
            libc::tcflush(libc::STDIN_FILENO, libc::TCIFLUSH);
        }
        let mut command = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut command) {
            eprintln!("read command fail: {}", error);
        }
        
        let (callback_tx, callback_rx) = oneshot::channel();
        // send fail, It indicates that the consumer has closed（That means the consumer is closed）
        if let Err(_) = tx.blocking_send((command, callback_tx)) {
            break;
        }
        if let Err(_) = callback_rx.blocking_recv() {
            break;
        }
    }
}

// start
pub(super) async fn run(mut rx: mpsc::Receiver<(String, oneshot::Sender<()>)>) {
    // let mut command;
    // let mut prompt = PROMPT_DEFAULT_COW;
    while let Some((command, collback_tx)) = rx.recv().await {
        // command = interactive::wait_command(&prompt);
        let mut command_iter = command.split_whitespace();
        let command_iter = &mut command_iter;
        let token = command_iter.next();
        let prompt = match token {
            Some("help") => help(),
            // 友好的退出
            // todo 监听 ctrl + c、kill等，在退出时执行相同的处理
            // todo 关闭正在执行的录音对象，清理tap等内容
            // 测试kill、ctrl + c 、painc 等场景下，结构体的drop方法是否会执行
            // painc 回执行drop方法，其它场景不会
            Some("quit") => {
                break;
            }
            Some("process") => process::run_command(command_iter),
            // 录音相关
            Some("re") => re::run_command(command_iter),
            _ => PROMPT_ERR_COMMAND_COW,
        };
        interactive::print_line(&prompt);
        let _ = collback_tx.send(());
    }
}

fn help() -> Cow<'static, str> {
    interactive::print_line(
        "if you want to view details for command, please use \"command help\"",
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
