use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use crate::ffi::{self, AVCodecID};
use crate::util::media;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodecId {
    NONE,
    // pcm 编解码器
    PCM_S16LE,
    PCM_S16BE,
    PCM_U16LE,
    PCM_U16BE,
    PCM_S8,
    PCM_U8,
    PCM_MULAW,
    PCM_ALAW,
    PCM_S32LE,
    PCM_S32BE,
    PCM_U32LE,
    PCM_U32BE,
    PCM_S24LE,
    PCM_S24BE,
    PCM_U24LE,
    PCM_U24BE,
    PCM_S24DAUD,
    PCM_ZORK,
    PCM_S16LE_PLANAR,
    PCM_DVD,
    PCM_F32BE,
    PCM_F32LE,
    PCM_F64BE,
    PCM_F64LE,
    PCM_BLURAY,
    PCM_LXF,
    S302M,
    PCM_S8_PLANAR,
    PCM_S24LE_PLANAR,
    PCM_S32LE_PLANAR,
    PCM_S16BE_PLANAR,

    PCM_S64LE,
    PCM_S64BE,

}

impl CodecId {
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from(ffi::avcodec_get_type((*self).into())) }
    }

    pub fn name(&self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr(ffi::avcodec_get_name((*self).into())).to_bytes()) }
    }
}

impl From<ffi::AVCodecID> for CodecId {
    fn from(value: ffi::AVCodecID) -> Self {
        match value {
            ffi::AVCodecID::AV_CODEC_ID_NONE => CodecId::NONE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S16LE => CodecId::PCM_S16LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S16BE => CodecId::PCM_S16BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U16LE => CodecId::PCM_U16LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U16BE => CodecId::PCM_U16BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S8 => CodecId::PCM_S8,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U8 => CodecId::PCM_U8,
            ffi::AVCodecID::AV_CODEC_ID_PCM_MULAW => CodecId::PCM_MULAW,
            ffi::AVCodecID::AV_CODEC_ID_PCM_ALAW => CodecId::PCM_ALAW,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S32LE => CodecId::PCM_S32LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S32BE => CodecId::PCM_S32BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U32LE => CodecId::PCM_U32LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U32BE => CodecId::PCM_U32BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S24LE => CodecId::PCM_S24LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S24BE => CodecId::PCM_S24BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U24LE => CodecId::PCM_U24LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_U24BE => CodecId::PCM_U24BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S24DAUD => CodecId::PCM_S24DAUD,
            ffi::AVCodecID::AV_CODEC_ID_PCM_ZORK => CodecId::PCM_ZORK,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S16LE_PLANAR => CodecId::PCM_S16LE_PLANAR,
            ffi::AVCodecID::AV_CODEC_ID_PCM_DVD => CodecId::PCM_DVD,
            ffi::AVCodecID::AV_CODEC_ID_PCM_F32BE => CodecId::PCM_F32BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_F32LE => CodecId::PCM_F32LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_F64BE => CodecId::PCM_F64BE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_F64LE => CodecId::PCM_F64LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_BLURAY => CodecId::PCM_BLURAY,
            ffi::AVCodecID::AV_CODEC_ID_PCM_LXF => CodecId::PCM_LXF,
            ffi::AVCodecID::AV_CODEC_ID_S302M => CodecId::S302M,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S8_PLANAR => CodecId::PCM_S8_PLANAR,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S24LE_PLANAR => CodecId::PCM_S24LE_PLANAR,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S32LE_PLANAR => CodecId::PCM_S32LE_PLANAR,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S16BE_PLANAR => CodecId::PCM_S16BE_PLANAR,

            ffi::AVCodecID::AV_CODEC_ID_PCM_S64LE => CodecId::PCM_S64LE,
            ffi::AVCodecID::AV_CODEC_ID_PCM_S64BE => CodecId::PCM_S64BE,

            _ => todo!(),
        }
    }
}

impl From<CodecId> for AVCodecID {
    fn from(value: CodecId) -> Self {
        match value {
            CodecId::NONE => ffi::AVCodecID::AV_CODEC_ID_NONE,
            CodecId::PCM_S16LE => ffi::AVCodecID::AV_CODEC_ID_PCM_S16LE,
            CodecId::PCM_S16BE => ffi::AVCodecID::AV_CODEC_ID_PCM_S16BE,
            CodecId::PCM_U16LE => ffi::AVCodecID::AV_CODEC_ID_PCM_U16LE,
            CodecId::PCM_U16BE => ffi::AVCodecID::AV_CODEC_ID_PCM_U16BE,
            CodecId::PCM_S8 => ffi::AVCodecID::AV_CODEC_ID_PCM_S8,
            CodecId::PCM_U8 => ffi::AVCodecID::AV_CODEC_ID_PCM_U8,
            CodecId::PCM_MULAW => ffi::AVCodecID::AV_CODEC_ID_PCM_MULAW,
            CodecId::PCM_ALAW => ffi::AVCodecID::AV_CODEC_ID_PCM_ALAW,
            CodecId::PCM_S32LE => ffi::AVCodecID::AV_CODEC_ID_PCM_S32LE,
            CodecId::PCM_S32BE => ffi::AVCodecID::AV_CODEC_ID_PCM_S32BE,
            CodecId::PCM_U32LE => ffi::AVCodecID::AV_CODEC_ID_PCM_U32LE,
            CodecId::PCM_U32BE => ffi::AVCodecID::AV_CODEC_ID_PCM_U32BE,
            CodecId::PCM_S24LE => ffi::AVCodecID::AV_CODEC_ID_PCM_S24LE,
            CodecId::PCM_S24BE => ffi::AVCodecID::AV_CODEC_ID_PCM_S24BE,
            CodecId::PCM_U24LE => ffi::AVCodecID::AV_CODEC_ID_PCM_U24LE,
            CodecId::PCM_U24BE => ffi::AVCodecID::AV_CODEC_ID_PCM_U24BE,
            CodecId::PCM_S24DAUD => ffi::AVCodecID::AV_CODEC_ID_PCM_S24DAUD,
            CodecId::PCM_ZORK => ffi::AVCodecID::AV_CODEC_ID_PCM_ZORK,
            CodecId::PCM_S16LE_PLANAR => ffi::AVCodecID::AV_CODEC_ID_PCM_S16LE_PLANAR,
            CodecId::PCM_DVD => ffi::AVCodecID::AV_CODEC_ID_PCM_DVD,
            CodecId::PCM_F32BE => ffi::AVCodecID::AV_CODEC_ID_PCM_F32BE,
            CodecId::PCM_F32LE => ffi::AVCodecID::AV_CODEC_ID_PCM_F32LE,
            CodecId::PCM_F64BE => ffi::AVCodecID::AV_CODEC_ID_PCM_F64BE,
            CodecId::PCM_F64LE => ffi::AVCodecID::AV_CODEC_ID_PCM_F64LE,
            CodecId::PCM_BLURAY => ffi::AVCodecID::AV_CODEC_ID_PCM_BLURAY,
            CodecId::PCM_LXF => ffi::AVCodecID::AV_CODEC_ID_PCM_LXF,
            CodecId::S302M => ffi::AVCodecID::AV_CODEC_ID_S302M,
            CodecId::PCM_S8_PLANAR => ffi::AVCodecID::AV_CODEC_ID_PCM_S8_PLANAR,
            CodecId::PCM_S24LE_PLANAR => ffi::AVCodecID::AV_CODEC_ID_PCM_S24LE_PLANAR,
            CodecId::PCM_S32LE_PLANAR => ffi::AVCodecID::AV_CODEC_ID_PCM_S32LE_PLANAR,
            CodecId::PCM_S16BE_PLANAR => ffi::AVCodecID::AV_CODEC_ID_PCM_S16BE_PLANAR,
            CodecId::PCM_S64LE => ffi::AVCodecID::AV_CODEC_ID_PCM_S64LE,
            CodecId::PCM_S64BE => ffi::AVCodecID::AV_CODEC_ID_PCM_S64BE
        }
    }
}


/// 根据codec_id获取编码名称
pub fn avcodec_get_name(id: CodecId) -> Option<String> {
    let name = unsafe { ffi::avcodec_get_name(id.into()) };
    if name.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(name) };
        Some(unsafe { from_utf8_unchecked(c_str.to_bytes()) }.to_string())
    }
}

