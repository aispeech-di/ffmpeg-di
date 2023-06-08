use std::ops::Deref;
use std::ptr;
use crate::codec::codec::Codec;
use crate::util::samplefmt::SampleFormat;
use crate::ffi;
use crate::util::channel_layout::ChannelLayout;

pub struct Audio {
    codec: Codec,
}


impl Audio {
    pub fn new(codec: Codec) -> Self {
        Audio { codec }
    }

    /// 编解码器支持的采样率列表
    pub fn sample_rates(&self) -> Option<RateIter> {
        unsafe {
            if (*self.as_ptr()).supported_samplerates.is_null() {
                None
            } else {
                Some(RateIter::new((*self.as_ptr()).supported_samplerates))
            }
        }
    }

    /// 编解码器支持的格式列表
    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            if (*self.as_ptr()).sample_fmts.is_null() {
                None
            } else {
                Some(FormatIter::new((*self.codec.as_ptr()).sample_fmts))
            }
        }
    }

    pub fn channel_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe {
            if (*self.as_ptr()).ch_layouts.is_null() {
                None
            } else {
                Some(ChannelLayoutIter::new((*self.as_ptr()).ch_layouts))
            }
        }
    }
}

impl Deref for Audio {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: *const i32,
}

impl RateIter {
    pub fn new(ptr: *const i32) -> Self {
        RateIter { ptr }
    }
}

impl Iterator for RateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let rate = *self.ptr;
            self.ptr = self.ptr.offset(1);

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: *const ffi::AVSampleFormat,
}

impl FormatIter {
    pub fn new(ptr:  *const ffi::AVSampleFormat) -> Self {
        FormatIter { ptr }
    }
}

impl Iterator for FormatIter {
    type Item = SampleFormat;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if *self.ptr == ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);
            Some(format)
        }
    }
}

pub struct ChannelLayoutIter {
    ptr:  *const ffi::AVChannelLayout,
}

impl ChannelLayoutIter {
    pub fn new(ptr: *const ffi::AVChannelLayout) -> Self {
        ChannelLayoutIter { ptr }
    }
}

impl Iterator for ChannelLayoutIter {
    type Item = ChannelLayout;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.ptr == ptr::null() {
                return None;
            }

            let layout = ChannelLayout::wrap(self.ptr as *mut _);
            self.ptr = self.ptr.offset(1);
            Some(layout)
        }
    }
}