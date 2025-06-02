//! record sound commond

use std::borrow::Cow;
use std::fs;

use audio::{
    AudioBufferList, AudioObjectId, AudioTimeStamp, OSStatus, aggregate_device, device,
    ext_audio_file, stream, tap,
};

use crate::interactive::{PROMPT_ERR_COMMOND_COW, print_list};

use crate::commond_ope;
use crate::rserror::{Result, RsError};

const DEFAULT_AGGREGATE_DEVICE_NAME: &str = "resound-aggregate-device";
const DEFAULT_AGGREGATE_DEVICE_UID: &str = "ABF64EB6-DC77-4251-80E2-1E773C25755E";
const DEFAULT_FILE_NAME: &str = "resound";

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
    [(Cow::Borrowed("start"), Cow::Borrowed("start record sound. usage: re start process_id"))],
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
    let aggregate_device = aggregate_device::AudioAggregateDevice::builder(
        DEFAULT_AGGREGATE_DEVICE_NAME,
        DEFAULT_AGGREGATE_DEVICE_UID,
    )
    .private(false)
    .tap_list(vec![tap_uid])
    .build()?;
    // 查询 stream
    // 读取stream 格式
    let streams = stream::list_by_id(&aggregate_device)?;
    if streams.is_empty() {
        return Err(RsError::with_msg("创建的临时aggregate device没有stream"))?;
    }
    // 一个stream,创建一个文件,暂时不考虑多个stream合并的问题
    // create audio file
    let mut audio_ext_file_vec: Vec<ext_audio_file::AudioExtAudioFile> =
        Vec::with_capacity(streams.len());
    for (i, stream) in streams.iter().enumerate() {
        match stream.get_basic_description() {
            Ok(basic_description) => {
                let audio_ext_file = ext_audio_file::AudioExtAudioFile::create(
                    format!("{}-{}.caf", DEFAULT_FILE_NAME, i),
                    basic_description,
                )?;
                audio_ext_file_vec.push(audio_ext_file);
            }
            Err(error) => {
                // clean audio file
                audio_ext_file_vec.iter().for_each(|path| {
                    let _ = fs::remove_file(path);
                });
                audio_ext_file_vec.iter().for_each(|path| {
                    let _ = fs::remove_file(path);
                });
                // println! error
                return Err(error)?;
            }
        }
    }
    // create io proc id
    let re_io_proc = ReIoProc {audio_ext_file_vec};
    let mut audio_io_proc_handler = device::AudioIoProcHandler::new(&aggregate_device, re_io_proc);
    // start
    audio_io_proc_handler.start()?;
    // 临时方案：
    // 调用audio模块的一个tap创建方法
    // 方法内完整实现整个流程，包含停应用
    // 逐步抽取代码到其他方法

    let ten_millis = std::time::Duration::from_millis(5000);
    std::thread::sleep(ten_millis);

    audio_io_proc_handler.stop()?;

    Ok(Cow::Borrowed("start record sound..."))
}

struct ReIoProc {
    audio_ext_file_vec: Vec<ext_audio_file::AudioExtAudioFile>,
}

impl device::AudioIoProc for ReIoProc {
    fn proc(
        &mut self,
        _in_device: AudioObjectId,
        _in_now: &AudioTimeStamp,
        in_input_data: &AudioBufferList,
        _in_input_time: &AudioTimeStamp,
        _out_output_data: &mut AudioBufferList,
        _in_output_time: &AudioTimeStamp,
    ) -> OSStatus {
        let m_number_buffers = in_input_data.mNumberBuffers;
        
        let mut all_success = true;

        for i in 0..m_number_buffers as usize {
            let buffer = in_input_data.mBuffers[i];
            let buffers = [buffer];
            // 目前，没有对stream的情况做任何处理
            // 一个stream对应一个音频文件
            // 需要重新组装AudioBufferList，满足格式要求
            let io_data = AudioBufferList{
                mNumberBuffers: 1,
                mBuffers: buffers,
            };
            if let Some(ext_audio_file) = self.audio_ext_file_vec.get_mut(i) {
                if let Err(error) = ext_audio_file.write_audio_buffer_list_async(&io_data) {
                    all_success = false;
                    eprintln!("{}", error);
                }
            }
        }

        if all_success {
            audio::K_AUDIO_HARDWARE_NO_ERROR
        } else {
            audio::K_AUDIO_HARDWARE_ILLEGAL_OPERATION_ERROR
        }
    }
}
