//! record sound

use std::borrow::Cow;

use audio::{tap, AudioObjectId};

use crate::interactive::{PROMPT_ERR_COMMOND_COW, print_list};

const TAP_NAME: &str = "resoundTap";

pub(super) fn run_commond<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let token = commond_iter.next();
    match token {
        Some("help") => help(),
        Some("start") => start(commond_iter),
        _ => PROMPT_ERR_COMMOND_COW,
    }
}

// start recond sound
fn start<'a, I>(commond_iter: &mut I) -> Cow<'_, str>
where
    I: Iterator<Item = &'a str>, // Item 是 &'a str，生命周期 'a 确保字符串切片有效
{
    let process_id;
    let token = commond_iter.next();
    if let Some(id) = token {
        // commond appoint process id
        // todo 需要检查ID是数字，需要检查ID存在，需要支持多个ID
        process_id = id.parse::<AudioObjectId>().unwrap();
    } else {
        // todo
        todo!()
    }
    // create tap
    let tap_desc_builder = tap::AudioTapDescriptionBuilder {
            name: TAP_NAME.to_string(),
            uid: None,
            processes: vec![process_id],
            mono: false,
            exclusive: false,
            mixdown: true,
            private: false,
            device_uid: None,
            stream: None,
    };
    let _ = tap_desc_builder.build();
    // create aggregate device
    // 读取stream 格式
    // create audio file
    // create io proc id
    // start
    // 临时方案：
    // 调用audio模块的一个tap创建方法
    // 方法内完整实现整个流程，包含停应用
    // 逐步抽取代码到其他方法
    Cow::Borrowed("start record sound...")
}

// show help
fn help() -> Cow<'static, str> {
    print_list(&HELP_CONTENT);
    crate::interactive::PROMPT_DEFAULT_COW
}

const HELP_CONTENT: [[(Cow<'_, str>, Cow<'_, str>); 1]; 2] = [
    [(Cow::Borrowed("help"), Cow::Borrowed("show this"))],
    [(Cow::Borrowed("listall"), Cow::Borrowed("show all process"))],
];
