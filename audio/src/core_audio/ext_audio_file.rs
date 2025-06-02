//! ExtAudioFileRef of core audio

use std::ptr;
use std::{fs, path};

use crate::AudioStreamBasicDescription;
use crate::aoerror::AudioError;
use crate::aoerror::Result;
use crate::foundation::create_cf_url_ref;
use coreaudio_sys::{AudioBufferList, ExtAudioFileRef};

/// encapsulation of ExtAudioFileRef
#[derive(Debug)]
pub struct AudioExtAudioFile {
    ext_audio_file_ref: ExtAudioFileRef,
    path: path::PathBuf,
}

impl AudioExtAudioFile {
    pub fn create<P: AsRef<path::Path>>(
        path_aef: P,
        stream_desc: &AudioStreamBasicDescription,
    ) -> Result<Self> {
        let path = path_aef.as_ref();
        if path.try_exists()? {
            return Err(AudioError::with_msg("文件已存在"))?;
        }
        // father path
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let cf_url = create_cf_url_ref(path, false)?;

        let mut ext_audio_file_ref = std::mem::MaybeUninit::<ExtAudioFileRef>::uninit();
        let status = unsafe {
            coreaudio_sys::ExtAudioFileCreateWithURL(
                cf_url,
                coreaudio_sys::kAudioFileCAFType,
                stream_desc,
                ptr::null(),
                coreaudio_sys::kAudioFileFlags_EraseFile,
                ext_audio_file_ref.as_mut_ptr(),
            )
        };
        check_status!("create ext audio file fail", status);
        let ext_audio_file_ref = unsafe { ext_audio_file_ref.assume_init() };

        Ok(AudioExtAudioFile {
            ext_audio_file_ref,
            path: path.to_path_buf(),
        })
    }

    pub fn write_audio_buffer_list_async(&mut self, io_data: &AudioBufferList) -> Result<()> {
        let float32_size = std::mem::size_of::<coreaudio_sys::Float32>() as u32;
        let buffer = &io_data.mBuffers[0];
        let number_frames_to_record = buffer.mDataByteSize / (buffer.mNumberChannels * float32_size);

        let status = unsafe { coreaudio_sys::ExtAudioFileWriteAsync(
            self.ext_audio_file_ref,
            number_frames_to_record, 
            io_data
        ) };
        check_status!("ext audio file write fail", status);
        Ok(())
    }
}

impl Drop for AudioExtAudioFile {
    fn drop(&mut self) {
        let status = unsafe { coreaudio_sys::ExtAudioFileDispose(self.ext_audio_file_ref) };
        eprintln_status!("core audio dispose ext audio file fail", status);
    }
}

impl AsRef<path::Path> for AudioExtAudioFile {
    fn as_ref(&self) -> &path::Path {
        self.path.as_path()
    }
}
