//! tap of core audio
//! multiple process or driver
//! 我的理解是，从进程、驱动上分叉，分出一个处理流程

use std::i32;

use coreaudio_sys::AudioObjectID;
use objc::{msg_send, runtime, sel, sel_impl};

use crate::{
    Result,
    aoerror::AudioError,
    foundation::{create_cf_array_ref, create_cf_number_ref, create_cf_string_ref},
};

/// AudioTapDescription builder
#[derive(Debug)]
pub struct AudioTapDescriptionBuilder {
    /// Human readable name of this tap.
    pub name: String,

    /// UID of this tap.
    /// It is usually a uuid
    pub uid: Option<String>,

    /// An NSArray of NSNumbers where each NSNumber holds the AudioObjectID of the process object to tap or exclude.
    pub processes: Vec<AudioObjectID>,

    /// True if this description is a mono mixdown of channels.
    pub mono: bool,

    /// True if this description should tap all processes except the process listed in the 'processes' property.
    pub exclusive: bool,

    /// True if this description is a mono or stereo mix of the tapped device's channels.
    pub mixdown: bool,

    /// True if this tap is only visible to the client process that created the tap.
    pub private: bool,

    // Set the tap's mute behavior. See CATapMuteBehavior above.
    // todo muteBehavior:
    /// An optional deviceUID that will have a value if this tap only taps a specific hardware device
    pub device_uid: Option<Vec<String>>,

    /// An optional NSNumber that will have a value if this tap taps a specific device stream.
    /// The value represents the index of the hardware stream.
    pub stream: Option<Vec<coreaudio_sys::AudioStreamID>>,
}

impl AudioTapDescriptionBuilder {
    pub fn build(self) -> Result<AudioTapDescription> {
        // 检查所有的processes，不能超过i32最大值
        // 在Objective-c的TapDescription中，要求processes是一个NSArray<NSNumber*>*，
        // 为了桥接方便，使用CFNumberRef代替，
        // AudioObjectId是u32，而CFNumberRef不支持u32，产生矛盾
        // 为了防止异常，限制processes中的值小于i32最大值
        // 为了防止内存泄漏、实现简单，先检查，再生成core foundation框架实例
        let no_supr_process_id = self
            .processes
            .iter()
            .find(|vlc_id| **vlc_id > i32::MAX as u32);
        if let Some(process_id) = no_supr_process_id {
            return Err(AudioError::with_msg(format!(
                "process id: {process_id} too big."
            )));
        }
        if self.name.is_empty() {
            return Err(AudioError::with_msg("name is must."));
        }
        if self.processes.is_empty() {
            return Err(AudioError::with_msg("processes is must."));
        }

        let name = create_cf_string_ref(&self.name);
        let process_id_cf_vec = self
            .processes
            .iter()
            .map(|&process_id| create_cf_number_ref(process_id as i32))
            .collect::<Vec<coreaudio_sys::CFNumberRef>>();
        let processes = create_cf_array_ref(&process_id_cf_vec);

        let description = unsafe {
            let cls = objc::class!(CATapDescription);
            let description: *mut runtime::Object = msg_send![cls, new];
            let _: () = msg_send!(description, setName : name);
            let _: () = msg_send!(description, setProcesses : processes);
            let _: () = msg_send!(description, setPrivate : self.private);
            let _: () = msg_send!(description, setMixdown : self.mixdown);
            let _: () = msg_send!(description, setMono : self.mono);
            let _: () = msg_send!(description, setExclusive : self.exclusive);
            description
        };

        Ok(AudioTapDescription {
            tap_description: description,
        })
    }
}

/// encapsulation of CATapDescription
/// use create tap
#[derive(Debug)]
pub struct AudioTapDescription {
    // macos CATapDescription
    tap_description: *mut runtime::Object,
}

impl Drop for AudioTapDescription {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.tap_description, release];
        }
    }
}

/// encapsulation of tap
#[derive(Debug)]
pub struct AudioTap {
    audio_object_id: AudioObjectID,
}

impl AudioTap {
    /// 创建 Process Tap
    pub fn new(desc: &AudioTapDescription) -> Result<Self> {
        let mut audio_object_id = 0;
        let status =
            unsafe { AudioHardwareCreateProcessTap(desc.tap_description, &mut audio_object_id) };
        check_status!("create process tap fail", status);
        Ok(AudioTap { audio_object_id })
    }
}

impl Drop for AudioTap {
    fn drop(&mut self) {
        let status = unsafe { AudioHardwareDestroyProcessTap(self.audio_object_id) };
        if status != super::K_AUDIO_HARDWARE_NO_ERROR {
            let status_msg = super::err_msg_hardware_status(status);
            eprintln!(
                "destroy process tap fail: {}[OSStatus: {}]",
                status_msg, status
            );
        }
    }
}

unsafe extern "C" {
    pub fn AudioHardwareCreateProcessTap(
        inDescription: *mut runtime::Object,
        outTapID: *mut AudioObjectID,
    ) -> coreaudio_sys::OSStatus;

    pub fn AudioHardwareDestroyProcessTap(inTapID: AudioObjectID) -> coreaudio_sys::OSStatus;
}
