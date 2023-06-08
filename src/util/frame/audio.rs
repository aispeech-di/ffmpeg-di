use std::{mem, slice};
use std::ops::{Deref, DerefMut};
use libc::c_int;
use crate::util::frame::Frame;
use crate::ffi;
use crate::util::channel_layout::ChannelLayout;
use crate::util::samplefmt::SampleFormat;

pub struct Audio(Frame);

impl Audio {
    pub unsafe fn wrap(ptr: *mut ffi::AVFrame) -> Self {
        Audio(Frame::wrap(ptr))
    }
}

impl Audio {
    #[inline(always)]
    pub fn empty() -> Self {
        Audio(Frame::empty())
    }

    #[inline]
    pub fn new(format: SampleFormat, samples: i32, layout: ChannelLayout) -> Self {
        let mut frame = Audio::empty();
        frame.alloc(format, samples, layout);
        frame
    }

    #[inline]
    pub fn alloc(&mut self, format: SampleFormat, samples: i32, layout: ChannelLayout) {
        unsafe {
            self.set_format(format);
            self.set_samples(samples);
            self.set_channel_layout(layout);

            // 分配音频缓存冲区
            ffi::av_frame_get_buffer(self.as_mut_ptr(), 0);
        }
    }

    #[inline]
    pub fn format(&self) -> SampleFormat {
        unsafe {
            if (*self.as_ptr()).format == -1 {
                SampleFormat::NONE
            } else {
                SampleFormat::from(mem::transmute::<_, ffi::AVSampleFormat>((*self.as_ptr()).format))
            }
        }
    }

    #[inline]
    pub fn set_format(&mut self, format: SampleFormat) {
        unsafe {
            (*self.as_mut_ptr()).format = mem::transmute::<ffi::AVSampleFormat, c_int>(format.into());
        }
    }

    #[inline]
    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).ch_layout = *value.as_ptr();
        }
    }

    #[inline]
    pub fn samples(&self) -> i32 {
        unsafe { (*self.as_ptr()).nb_samples }
    }

    #[inline]
    pub fn set_samples(&mut self, samples: i32) {
        unsafe {
            (*self.as_mut_ptr()).nb_samples = samples;
        }
    }

    #[inline]
    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::wrap(Box::into_raw(Box::new((*self.as_ptr()).ch_layout))) }
    }

    #[inline]
    pub fn channels(&self) -> i32 {
        self.channel_layout().nb_channels()
    }

    #[inline]
    pub fn sample_rate(&self) -> i32 {
        unsafe { (*self.as_ptr()).sample_rate }
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: i32) {
        unsafe {
            (*self.as_mut_ptr()).sample_rate = sample_rate;
        }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        self.format().is_planar()
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        self.format().is_packed()
    }


    /// 平面数
    #[inline]
    pub fn planes(&self) -> usize {
        unsafe {
            // 没有数据的时候为0
            if (*self.as_ptr()).linesize[0] == 0 {
                return 0;
            }
        }
        if self.is_packed() {
            1
        } else {
            self.channels() as usize
        }
    }

    #[inline]
    pub fn data<T: Sample>(&self, index: usize) -> &[T] {
        if index >= self.planes() {
            panic!("index out of range");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        if self.is_planar() {
            unsafe {
                slice::from_raw_parts((*self.as_ptr()).data[index] as *const T, self.samples() as usize)
            }
        } else {
            unsafe {
                slice::from_raw_parts((*self.as_ptr()).data[0] as *const T, (self.samples() * self.channels()) as usize)
            }
        }
    }


    #[inline]
    pub fn packed<T: Sample>(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).data[0] as *const T,
                (self.samples() * self.channels()) as usize,
            )
        }
    }

    #[inline]
    pub fn planar<T: Sample>(&self, index: usize) -> &[T] {
        unsafe {
            slice::from_raw_parts((*self.as_ptr()).data[index] as *const T, self.samples() as usize)
        }
    }
}

impl Deref for Audio {
    type Target = Frame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Audio {
    fn clone(&self) -> Self {
        unsafe {
            let mut cloned = Audio::new(self.format(), self.samples(), self.channel_layout());
            ffi::av_frame_copy(cloned.as_mut_ptr(), self.as_ptr());
            ffi::av_frame_copy_props(cloned.as_mut_ptr(), self.as_ptr());
            cloned
        }
    }
}

impl From<Frame> for Audio {
    fn from(frame: Frame) -> Self {
        Audio(frame)
    }
}

pub unsafe trait Sample {
    fn is_valid(format: SampleFormat, channels: i32) -> bool;
}

unsafe impl Sample for u8 {
    #[inline(always)]
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::U8 | SampleFormat::U8P)
    }
}

unsafe impl Sample for (u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::U8P
    }
}

unsafe impl Sample for i16 {
    #[inline(always)]
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::S16 | SampleFormat::S16P)
    }
}

unsafe impl Sample for (i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for (i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for (i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::S16P
    }
}

unsafe impl Sample for i32 {
    #[inline(always)]
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::S32 | SampleFormat::S32P)
    }
}

unsafe impl Sample for (i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for (i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for (i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::S32P
    }
}

unsafe impl Sample for i64 {
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::S64 | SampleFormat::S64P)
    }
}

unsafe impl Sample for (i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for (i64, i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for (i64, i64, i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for (i64, i64, i64, i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for (i64, i64, i64, i64, i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for (i64, i64, i64, i64, i64, i64, i64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::S64P
    }
}

unsafe impl Sample for f32 {
    #[inline(always)]
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::FLT | SampleFormat::FLTP)
    }
}

unsafe impl Sample for (f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for (f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for (f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::FLTP
    }
}

unsafe impl Sample for f64 {
    #[inline(always)]
    fn is_valid(format: SampleFormat, _channels: i32) -> bool {
        matches!(format, SampleFormat::DBL | SampleFormat::DBLP)
    }
}

unsafe impl Sample for (f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 2 && format == SampleFormat::DBLP
    }
}

unsafe impl Sample for (f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 3 && format == SampleFormat::DBLP
    }
}

unsafe impl Sample for (f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 4 && format == SampleFormat::DBLP
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 5 && format == SampleFormat::DBLP
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 6 && format == SampleFormat::DBLP
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: SampleFormat, channels: i32) -> bool {
        channels == 7 && format == SampleFormat::DBLP
    }
}


