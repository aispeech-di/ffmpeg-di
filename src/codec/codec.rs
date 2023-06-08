use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use anyhow::{anyhow, Result};
use crate::codec::audio::Audio;
use crate::codec::capabilities::Capabilities;

use crate::codec::codec_id::CodecId;
use crate::ffi;
use crate::util::media;

unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

pub struct Codec {
    ptr: *mut ffi::AVCodec,
}

impl Codec {
    pub unsafe fn wrap(ptr: *mut ffi::AVCodec) -> Self {
        Codec { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVCodec {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVCodec {
        self.ptr
    }
}

impl Codec {
    /// 是否为解码器
    pub fn is_decoder(&self) -> bool {
        unsafe {
            ffi::av_codec_is_decoder(self.as_ptr()) != 0
        }
    }

    /// 是否为解码器
    pub fn is_encoder(&self) -> bool {
        unsafe { ffi::av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn audio(self) -> Result<Audio> {
        if self.medium() == media::Type::Audio {
            Ok(Audio::new(self))
        } else {
            Err(anyhow!("not audio"))
        }
    }


    /// 编解码器名称
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    /// 编解码器的长名
    pub fn long_name(&self) -> &str {
        unsafe {
            let long_name = (*self.as_ptr()).long_name;
            if long_name.is_null() {
                ""
            } else {
                from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
            }
        }
    }

    /// 解码器媒体类型
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).type_) }
    }

    /// 编解码器编号
    pub fn id(&self) -> CodecId {
        unsafe { CodecId::from((*self.as_ptr()).id) }
    }

    /// 用于描述编解码器的能力和特性
    pub fn capabilities(&self) -> Capabilities {
        unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    }
}







