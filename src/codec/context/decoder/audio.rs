use std::ops::{Deref, DerefMut};
use crate::codec::context::Context;
use crate::util::channel_layout::ChannelLayout;
use crate::util::samplefmt::SampleFormat;
use super::Decoder;

pub struct Audio(pub Decoder);

impl Audio {
    pub fn rate(&self) -> i32 {
        unsafe { (*self.as_ptr()).sample_rate }
    }

    pub fn channels(&self) -> i32 {
        self.channel_layout().nb_channels()
    }

    pub fn format(&self) -> SampleFormat {
        unsafe {
            SampleFormat::from((*self.as_ptr()).sample_fmt)
        }
    }

    pub fn frames(&self) -> i64 {
        unsafe { (*self.as_ptr()).frame_num }
    }

    pub fn align(&self) -> i32 {
        unsafe { (*self.as_ptr()).block_align }
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe {
            ChannelLayout::wrap(&(*self.as_ptr()).ch_layout as *const _ as *mut _)
        }
    }

    //  参数指定了每个音频帧的采样数，用于解码器对压缩的音频数据进行解码
    pub fn frame_size(&self) -> i32 {
        unsafe { (*self.as_ptr()).frame_size }
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).ch_layout = *(value.as_ptr());
        }
    }

    pub fn set_request_sample_fmt(&mut self, value: SampleFormat) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }
}

impl Deref for Audio {
    type Target = Decoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}