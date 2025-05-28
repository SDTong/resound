//! record sound commond

use std::borrow::Cow;

use audio::{aggregate_device, tap, AudioObjectId};

use crate::interactive::{PROMPT_ERR_COMMOND_COW, print_list};

use crate::commond_ope;
use crate::rserror::Result;

const DEFAULT_AGGREGATE_DEVICE_NAME: &str = "resound-aggregate-device";
const DEFAULT_AGGREGATE_DEVICE_UID: &str = "ABF64EB6-DC77-4251-80E2-1E773C25755E";

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

    match recond_sound(process_id) {
        Ok(cow) => cow,
        Err(error) => Cow::from(error.to_string()),
    }
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

// recond sound
fn recond_sound(process_id: AudioObjectId) -> Result<Cow<'static, str>> {
    // create tap
    let tap_description_builder = tap::AudioTapDescriptionBuilder {
        name: commond_ope::TAP_NAME_DEFAULT.to_string(),
        uid: None,
        processes: vec![process_id],
        mono: false,
        exclusive: false,
        mixdown: true,
        private: false,
        device_uid: None,
        stream: None,
    };
    let tap_description = tap_description_builder.build()?;
    let tap = tap::AudioTap::create(&tap_description)?;
    let tap_uid = tap::query_uid(&tap)?;
    println!("tap_uid: {}", tap_uid);
    // create aggregate device
    let aggregate_device = aggregate_device::AudioAggregateDevice::builder(DEFAULT_AGGREGATE_DEVICE_NAME, DEFAULT_AGGREGATE_DEVICE_UID)
        .private(false)
        .tap_list(vec![tap_uid])
        .build()?;
    println!("aggregate_device: {:?}", aggregate_device);
    // 读取stream 格式
    // create audio file
    // create io proc id
    // start
    // 临时方案：
    // 调用audio模块的一个tap创建方法
    // 方法内完整实现整个流程，包含停应用
    // 逐步抽取代码到其他方法

    Ok(Cow::Borrowed("start record sound..."))
}
