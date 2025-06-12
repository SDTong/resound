//! 录系统播放的声音
//!
//! 交互式命令，通常情况下，格式为： 对象 + 动作 + 参数

use std::thread;

use tokio::{
    signal::unix::{SignalKind, signal},
    sync::{mpsc, oneshot},
};

mod command;
mod interactive;
mod rserror;

#[tokio::main]
async fn main() {
    // 监听默认 kill pid
    let kill_stream = signal(SignalKind::terminate());
    if let Err(_) = kill_stream {
        eprintln!("register signal SIGTERM fail");
        return;
    }
    // 监听 ctrl + c
    let ctrl_c_stream = signal(SignalKind::interrupt());
    if let Err(_) = ctrl_c_stream {
        eprintln!("register signal SIGINT fail");
        return;
    }
    let mut kill_stream = unsafe { kill_stream.unwrap_unchecked() };
    let mut ctrl_c_stream = unsafe { ctrl_c_stream.unwrap_unchecked() };

    let (tx, rx) = mpsc::channel(1);
    let signal_tx = tx.clone();

    // wait user input
    thread::spawn(|| {
        let prompt = &interactive::PROMPT_DEFAULT_COW;
        interactive::print_line(prompt);
        command::wait_command(tx);
    });

    let (callback_tx, callback_rx) = oneshot::channel::<()>();
    tokio::select! {
        // start main task
        _ = command::run(rx) => {}
        _ = ctrl_c_stream.recv() => {
            let _  = signal_tx.send(("quit".to_string(), callback_tx)).await;
            // wait main task finish
            let _ = callback_rx.await;
        }
        _ = kill_stream.recv() => {
            let _  = signal_tx.send(("quit".to_string(), callback_tx)).await;
            // wait main task finish
            let _ = callback_rx.await;
        }
    }
}
