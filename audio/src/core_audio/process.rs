//! Process of core audio
//! 感觉是支持音频的进程

use std::cell::OnceCell;

use coreaudio_sys::AudioObjectID;

use crate::{error::Result, get_or_try_init};

use super::{build_property_address, get_property_data_list, get_property_data_string};

/// encapsulation of process
/// coreAudio中的Process不支持name属性，可以查询对应进程ID(pid)的name属性
#[derive(Debug)]
pub struct AudioProcess {
    id: AudioObjectID,
    // bundle： 包，
    // 通常是反向域名表示法（Reverse Domain Name Notation）
    bundle_id: OnceCell<String>,
    // name: String,
}

impl AudioProcess {
    pub fn get_id(&self) -> AudioObjectID {
        self.id
    }
    /// get bundle id
    /// lazy loading
    /// If it is run for the first time, the bundle will be query;
    /// otherwise, the previous query results will be returned
    pub fn get_bundle_id(&self) -> Result<&String> {
        get_or_try_init(&self.bundle_id, || bundle_id(self.id))
    }
}

impl From<AudioObjectID> for AudioProcess {
    fn from(id: AudioObjectID) -> Self {
        AudioProcess {
            id,
            bundle_id: OnceCell::new(),
        }
    }
}

/// find all process
/// only init id, other value lazy loading
pub fn list() -> Result<Vec<AudioProcess>> {
    list_with_id(coreaudio_sys::kAudioObjectSystemObject)
}

// find process by id
// only init id, other value lazy loading
fn list_with_id(id: AudioObjectID) -> Result<Vec<AudioProcess>> {
    let id_vec = list_id_with_id(id)?;
    let audio_process_vec = id_vec
        .into_iter()
        .map(|id| AudioProcess::from(id))
        .collect();
    Ok(audio_process_vec)
}

// find process id by id
#[inline]
fn list_id_with_id(id: AudioObjectID) -> Result<Vec<AudioObjectID>> {
    let addr = build_property_address(coreaudio_sys::kAudioHardwarePropertyProcessObjectList);
    get_property_data_list(id, &addr)
}

// 查询 bundle id
fn bundle_id(id: AudioObjectID) -> Result<String> {
    let addr = build_property_address(coreaudio_sys::kAudioProcessPropertyBundleID);
    get_property_data_string(id, &addr)
}
