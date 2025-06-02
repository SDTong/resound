//! device of core auido

use std::{ffi, panic};

use coreaudio_sys::{
    AudioBufferList, AudioDeviceCreateIOProcID, AudioDeviceDestroyIOProcID, AudioDeviceID,
    AudioDeviceStart, AudioDeviceStop, AudioObjectID, AudioTimeStamp, OSStatus,
};

use crate::aoerror::Result;

/// encapsulation of AudioDeviceIOProc
/// 与AudioIoProcHandler区别：AudioIoProc用于调用者实现自己的处理逻辑
pub trait AudioIoProc {
    fn proc(
        &mut self,
        in_device: AudioObjectID,
        in_now: &AudioTimeStamp,
        in_input_data: &AudioBufferList,
        in_input_time: &AudioTimeStamp,
        out_output_data: &mut AudioBufferList,
        in_output_time: &AudioTimeStamp,
    ) -> OSStatus;
}

/// encapsulation of AudioDeviceIOProcId
/// 与AudioIoProc区别： AudioIoProcHandler是Audio的struct，用于生命周期控制
pub struct AudioIoProcHandler<T: AudioIoProc> {
    // device id
    audio_device_id: AudioDeviceID,
    // 调用者执行的方法
    audio_io_proc: T,
    io_proc_id: coreaudio_sys::AudioDeviceIOProcID,
    // true: running false: no running
    is_run: bool,
}

impl<T: AudioIoProc> AudioIoProcHandler<T> {
    pub fn new(audio_device_id: &AudioDeviceID, audio_io_proc: T) -> AudioIoProcHandler<T> {
        AudioIoProcHandler {
            audio_device_id: *audio_device_id,
            audio_io_proc,
            io_proc_id: None,
            is_run: false,
        }
    }

    // AudioDeviceCreateIOProcID
    fn init_io_proc_id(&mut self) -> Result<()> {
        let mut out_ioproc_id =
            std::mem::MaybeUninit::<coreaudio_sys::AudioDeviceIOProcID>::uninit();
        let status = unsafe {
            AudioDeviceCreateIOProcID(
                self.audio_device_id,
                Some(Self::audio_io_proc_trampoline),
                &mut self.audio_io_proc as *mut T as *mut ffi::c_void,
                out_ioproc_id.as_mut_ptr(),
            )
        };
        check_status!("core audio create io proc fail", status);
        self.io_proc_id = unsafe { out_ioproc_id.assume_init() };

        Ok(())
    }

    // AudioDeviceStart
    pub fn start(&mut self) -> Result<()> {
        if self.is_run {
            return Ok(());
        }
        if let None = self.io_proc_id {
            // no init_io_proc_id, do init_io_proc_id
            self.init_io_proc_id()?;
        }
        let status = unsafe { AudioDeviceStart(self.audio_device_id, self.io_proc_id) };
        check_status!("core audio start io proc fail", status);

        self.is_run = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.is_run {
            return Ok(());
        }

        let status = unsafe { AudioDeviceStop(self.audio_device_id, self.io_proc_id) };
        check_status!("core audio stop io proc fail", status);
        self.is_run = false;

        Ok(())
    }

    unsafe extern "C" fn audio_io_proc_trampoline(
        in_device: AudioObjectID,
        in_now: *const AudioTimeStamp,
        in_input_data: *const AudioBufferList,
        in_input_time: *const AudioTimeStamp,
        out_output_data: *mut AudioBufferList,
        in_output_time: *const AudioTimeStamp,
        in_client_data: *mut ffi::c_void,
    ) -> OSStatus {
        if in_client_data.is_null() {
            // 应该记录错误，或者根据 Core Audio 的要求返回特定的错误码
            eprintln!("Error: inClientData is null in derive io proc!");
            return coreaudio_sys::kAudioHardwareUnspecifiedError as OSStatus;
        }

        // 将 inClientData 转换回 &mut T
        let audio_io_proc = unsafe { &mut *(in_client_data as *mut T) };
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            unsafe {
                audio_io_proc.proc(
                    in_device,
                    &(*in_now),
                    &(*in_input_data),
                    &(*in_input_time),
                    &mut (*out_output_data),
                    &(*in_output_time),
                )
            }
        }));

        match result {
            Ok(status) => status,
            Err(error) => {
                if let Some(s) = error.downcast_ref::<&str>() {
                    eprintln!("Panic occurred in audio_io_proc: {}", s);
                } else {
                    eprintln!("Panic occurred in audio_io_proc!");
                }
                coreaudio_sys::kAudioHardwareUnspecifiedError as OSStatus
            }
        }
    }
}

impl<T: AudioIoProc> Drop for AudioIoProcHandler<T> {
    fn drop(&mut self) {
        if self.is_run {
            let _ = self.stop();
        }
        if self.io_proc_id.is_some() {
            let status =
                unsafe { AudioDeviceDestroyIOProcID(self.audio_device_id, self.io_proc_id) };
            eprintln_status!("core audio destroy io proc fail", status);
        }
    }
}
