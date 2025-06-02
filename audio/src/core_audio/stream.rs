//! stream of core audio

use std::cell::OnceCell;
use std::mem;

use coreaudio_sys::{AudioObjectID, AudioStreamID};

use crate::aoerror::Result;
use crate::core_audio::{build_property_address, get_property_data_list};
use crate::{AudioStreamBasicDescription, get_or_try_init};

/// encapsulation of stream
#[derive(Debug)]
pub struct AudioStream {
    audio_stream_id: AudioStreamID,
    // stream 格式
    basic_description: OnceCell<AudioStreamBasicDescription>,
}

impl AudioStream {
    pub fn get_basic_description(&self) -> Result<&AudioStreamBasicDescription> {
        get_or_try_init(&self.basic_description, || {
            basic_description(&self.audio_stream_id)
        })
    }
}

impl From<AudioStreamID> for AudioStream {
    fn from(value: AudioStreamID) -> Self {
        AudioStream {
            audio_stream_id: value,
            basic_description: OnceCell::new(),
        }
    }
}

/// find streams by AudioObjectId
/// only init id, other value lazy loading
pub fn list_by_id(id: &AudioObjectID) -> Result<Vec<AudioStream>> {
    let id_vec = list_id_by_id(id)?;
    let audio_stream_vec = id_vec.into_iter().map(|id| AudioStream::from(id)).collect();
    Ok(audio_stream_vec)
}

// find process id by id
#[inline]
fn list_id_by_id(id: &AudioObjectID) -> Result<Vec<AudioObjectID>> {
    let addr = build_property_address(coreaudio_sys::kAudioDevicePropertyStreams);
    get_property_data_list(*id, &addr)
}

// query stream basic description
fn basic_description(audio_stream_id: &AudioStreamID) -> Result<AudioStreamBasicDescription> {
    let addr = build_property_address(coreaudio_sys::kAudioStreamPropertyVirtualFormat);
    let mut basic_description = mem::MaybeUninit::<AudioStreamBasicDescription>::uninit();
    let mut size = mem::size_of::<AudioStreamBasicDescription>() as coreaudio_sys::UInt32;
    let status = unsafe {
        coreaudio_sys::AudioObjectGetPropertyData(
            *audio_stream_id,
            &addr,
            0,
            std::ptr::null(),
            &mut size,
            basic_description.as_mut_ptr() as *mut std::ffi::c_void,
        )
    };
    check_status!("query stream basic description fail", status);
    let basic_description = unsafe { basic_description.assume_init() };
    Ok(basic_description)
}
