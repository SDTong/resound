//! AggregateDevice of core audio

use std::ffi::c_void;

use coreaudio_sys::{
    AudioDeviceID, AudioHardwareCreateAggregateDevice, AudioHardwareDestroyAggregateDevice,
    CFDictionaryRef,
};

use crate::{
    aoerror::Result,
    foundation::{
        create_cf_array_ref, create_cf_dictionary_ref, create_cf_number_ref,
        create_cf_string_ref,
    },
};

use super::tap;

// key start
// coreaudio-sys 绑定的key，都是c风格的，用于创建CFStringRef时，多一个\0字符
// kAudioAggregateDeviceNameKey
const K_AUDIO_AGGREGATE_DEVICE_NAME_KEY: &str = "name";
// kAudioAggregateDeviceUIDKey
const K_AUDIO_AGGREGATE_DEVICE_UIDKEY: &str = "uid";
// kAudioAggregateDeviceMainSubDeviceKey
const K_AUDIO_AGGREGATE_DEVICE_MAIN_SUB_DEVICE_KEY: &str = "master";
// kAudioAggregateDeviceIsPrivateKey
const K_AUDIO_AGGREGATE_DEVICE_IS_PRIVATE_KEY: &str = "private";
// kAudioAggregateDeviceTapListKey
const K_AUDIO_AGGREGATE_DEVICE_TAP_LIST_KEY: &str = "taps";
// key end

/// AggregateDevice Builder
#[derive(Debug)]
pub struct AudioAggregateDeviceBuilder {
    name: String,
    uid: String,

    // The value for this key is a CFString that contains the
    // UID for the sub-device that is the time source for the AudioAggregateDevice.
    main_sub_device: Option<String>,

    // The value for this key is a CFNumber where a value of 0
    // means that the AudioAggregateDevice is to be published to the entire system and
    // a value of 1 means that the AudioAggregateDevice is private to the process that
    // created it. Note that a private AudioAggregateDevice is not persistent across
    // launches of the process that created it. Note that if this key is not present,
    // it implies that the AudioAggregateDevice is published to the entire system.
    private: Option<bool>,

    // core audio 框架要求传入一个 CFDictionaries 的 CFArray。
    // 目前，CFDictionaries 的 key 只知道一个： uid ， 所以使用 Vec<String> 表示
    tap_list: Option<Vec<String>>,
}

impl AudioAggregateDeviceBuilder {
    fn new<T: Into<String>>(name: T, uid: T) -> AudioAggregateDeviceBuilder {
        AudioAggregateDeviceBuilder {
            name: name.into(),
            uid: uid.into(),
            main_sub_device: None,
            private: None,
            tap_list: None,
        }
    }

    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }

    pub fn tap_list(mut self, tap_list: Vec<String>) -> Self {
        self.tap_list = Some(tap_list);
        self
    }

    pub fn build(self) -> Result<AudioAggregateDevice> {
        AudioAggregateDevice::create(&self)
    }
}

/// encapsulation of tap
#[derive(Debug)]
pub struct AudioAggregateDevice {
    /// false: 当实例释放时，不删除device, true: 当实例释放时，删除device
    destroy: bool,
    audio_object_id: AudioDeviceID,
}

impl AudioAggregateDevice {
    #[inline]
    pub fn builder<T: Into<String>>(name: T, uid: T) -> AudioAggregateDeviceBuilder {
        AudioAggregateDeviceBuilder::new(name, uid)
    }

    fn create(builder: &AudioAggregateDeviceBuilder) -> Result<AudioAggregateDevice> {
        let mut keys = Vec::with_capacity(2);
        let mut values = Vec::with_capacity(2);

        keys.push(create_cf_string_ref(K_AUDIO_AGGREGATE_DEVICE_NAME_KEY));
        values.push(create_cf_string_ref(&builder.name) as *const c_void);
        keys.push(create_cf_string_ref(K_AUDIO_AGGREGATE_DEVICE_UIDKEY));
        values.push(create_cf_string_ref(&builder.uid) as *const c_void);

        if let Some(main_sub_device) = &builder.main_sub_device {
            keys.push(create_cf_string_ref(
                K_AUDIO_AGGREGATE_DEVICE_MAIN_SUB_DEVICE_KEY,
            ));
            values.push(create_cf_string_ref(main_sub_device) as *const c_void);
        }
        if let Some(private) = builder.private {
            keys.push(create_cf_string_ref(
                K_AUDIO_AGGREGATE_DEVICE_IS_PRIVATE_KEY,
            ));
            let private = if private { 1 } else { 0 };
            values.push(create_cf_number_ref(private) as *const c_void);
        }
        if let Some(tap_list) = &builder.tap_list {
            let tap_list = tap_list
                .iter()
                .map(|tap_uid| {
                    let mut keys = [create_cf_string_ref(tap::K_AUDIO_SUB_TAP_UIDKEY)];
                    let mut values = [create_cf_string_ref(tap_uid)];
                    create_cf_dictionary_ref(&mut keys, &mut values)
                })
                .collect::<Vec<CFDictionaryRef>>();
            let tap_list = create_cf_array_ref(&tap_list);
            keys.push(create_cf_string_ref(K_AUDIO_AGGREGATE_DEVICE_TAP_LIST_KEY));
            values.push(tap_list as *const c_void);
        }
        let in_description = create_cf_dictionary_ref(&mut keys, &mut values);

        let mut aggregate_device_id = 0;
        let status =
            unsafe { AudioHardwareCreateAggregateDevice(in_description, &mut aggregate_device_id) };
        check_status!("create aggregate device fail", status);
        Ok(AudioAggregateDevice {
            destroy: true,
            audio_object_id: aggregate_device_id,
        })
    }
}

// todo 怎么保证 AggregateDevice 删除前，tap不会被删除
impl Drop for AudioAggregateDevice {
    fn drop(&mut self) {
        if self.destroy {
            let status = unsafe { AudioHardwareDestroyAggregateDevice(self.audio_object_id) };
            eprintln_status!("destroy process tap fail", status);
        }
    }
}
