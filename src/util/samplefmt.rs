use std::ffi::{CStr, CString};
use std::str::from_utf8_unchecked;
use crate::ffi;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SampleFormat {
    NONE,
    U8,
    S16,
    S32,
    S64,
    FLT,
    DBL,
    U8P,
    S16P,
    S32P,
    S64P,
    FLTP,
    DBLP,
}

impl SampleFormat {
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(ffi::av_get_sample_fmt_name((*self).into())).to_bytes())
        }
    }

    #[inline]
    pub fn packed(&self) -> Self {
        unsafe { SampleFormat::from(ffi::av_get_packed_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn planar(&self) -> Self {
        unsafe { SampleFormat::from(ffi::av_get_planar_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { ffi::av_sample_fmt_is_planar((*self).into()) == 1 }
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        !self.is_planar()
    }

    /// 每个样本的字节数
    #[inline]
    pub fn bytes(&self) -> usize {
        unsafe { ffi::av_get_bytes_per_sample((*self).into()) as usize }
    }
}

impl From<ffi::AVSampleFormat> for SampleFormat {
    #[inline]
    fn from(value: ffi::AVSampleFormat) -> Self {
        match value {
            ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE => SampleFormat::NONE,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_U8 => SampleFormat::U8,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S16 => SampleFormat::S16,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S32 => SampleFormat::S32,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S64 => SampleFormat::S64,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT => SampleFormat::FLT,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_DBL => SampleFormat::DBL,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_U8P => SampleFormat::U8P,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S16P => SampleFormat::S16P,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S32P => SampleFormat::S32P,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_S64P => SampleFormat::S64P,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLTP => SampleFormat::FLTP,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_DBLP => SampleFormat::DBLP,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_NB => SampleFormat::NONE,
        }
    }
}

impl From<&'static str> for SampleFormat {
    #[inline]
    fn from(value: &'static str) -> Self {
        unsafe {
            let value = CString::new(value).unwrap();
            SampleFormat::from(ffi::av_get_sample_fmt(value.as_ptr()))
        }
    }
}


impl From<SampleFormat> for ffi::AVSampleFormat {
    fn from(value: SampleFormat) -> Self {
        match value {
            SampleFormat::NONE => ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE,

            SampleFormat::U8 => ffi::AVSampleFormat::AV_SAMPLE_FMT_U8,
            SampleFormat::S16 => ffi::AVSampleFormat::AV_SAMPLE_FMT_S16,
            SampleFormat::S32 => ffi::AVSampleFormat::AV_SAMPLE_FMT_S32,
            SampleFormat::S64 => ffi::AVSampleFormat::AV_SAMPLE_FMT_S64,
            SampleFormat::FLT => ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
            SampleFormat::DBL => ffi::AVSampleFormat::AV_SAMPLE_FMT_DBL,

            SampleFormat::U8P => ffi::AVSampleFormat::AV_SAMPLE_FMT_U8P,
            SampleFormat::S16P => ffi::AVSampleFormat::AV_SAMPLE_FMT_S16P,
            SampleFormat::S32P => ffi::AVSampleFormat::AV_SAMPLE_FMT_S32P,
            SampleFormat::S64P => ffi::AVSampleFormat::AV_SAMPLE_FMT_S64P,
            SampleFormat::FLTP => ffi::AVSampleFormat::AV_SAMPLE_FMT_FLTP,
            SampleFormat::DBLP => ffi::AVSampleFormat::AV_SAMPLE_FMT_DBLP,
        }
    }
}