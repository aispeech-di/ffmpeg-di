use std::mem;
use std::ops::{Deref};
use libc::c_int;
use crate::codec::codec_par::Parameters;
use crate::ffi;
use crate::util::rational::Rational;

use crate::format::context::Context;
use crate::format::disposition::Disposition;
use crate::format::packet::side_data::PacketSideData;
use crate::util::dict::{Dictionary, DictRef};
use crate::util::discard::Discard;

unsafe impl<'a> Send for Stream<'a> {}

pub struct Stream<'a> {
    context: &'a Context,
    index: i32,
}

impl<'a> Stream<'a> {
    pub unsafe fn wrap(context: &Context, index: i32) -> Stream {
        Stream { context, index }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVStream {
        *(*self.context.as_ptr()).streams.add(self.index as usize)
    }
}

impl<'a> Stream<'a> {
    pub fn index(&self) -> usize {
        unsafe { (*self.as_ptr()).index as usize }
    }

    pub fn id(&self) -> i32 { unsafe { (*self.as_ptr()).id } }

    pub fn parameters(&self) -> Parameters {
        unsafe { Parameters::wrap((*self.as_ptr()).codecpar, Some(self.context.destroy())) }
    }

    pub fn time_base(&self) -> Rational { unsafe { Rational::from((*self.as_ptr()).time_base) } }

    pub fn start_time(&self) -> i64 { unsafe { (*self.as_ptr()).start_time } }

    // duration 字段表示该流的总时长，以时间基为单位。时间基是一个结构体，包含分子和分母，用于表示时间的单位和精度。
    pub fn duration(&self) -> i64 { unsafe { (*self.as_ptr()).duration } }

    pub fn nb_frames(&self) -> i64 { unsafe { (*self.as_ptr()).nb_frames } }

    pub fn disposition(&self) -> Disposition {
        unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
    }

    /// 丢弃媒体策略
    pub fn discard(&self) -> Discard {
        unsafe { Discard::from((*self.as_ptr()).discard) }
    }

    /// 用于表示存储在 AVPacket 中的附加数据的类型
    pub fn side_data(&self) -> PacketSideDataIter {
        PacketSideDataIter::new(self)
    }

    pub fn frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
    }

    pub fn avg_frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
    }

    pub fn metadata(&self) -> DictRef {
        unsafe {
            DictRef::wrap((*self.as_ptr()).metadata)
        }
    }
}

impl<'a> PartialEq for Stream<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ptr() == other.as_ptr() }
    }
}


pub struct PacketSideDataIter<'a> {
    stream: &'a Stream<'a>,
    cur: c_int,
}

impl<'a> PacketSideDataIter<'a> {
    pub fn new<'sd, 's: 'sd>(stream: &'s Stream) -> PacketSideDataIter<'s> {
        PacketSideDataIter {
            stream,
            cur: 0,
        }
    }
}

impl<'a> Iterator for PacketSideDataIter<'a> {
    type Item = PacketSideData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.cur >= (*self.stream.as_ptr()).nb_side_data {
                return None;
            }

            self.cur += 1;
            Some(PacketSideData::wrap((*self.stream.as_ptr()).side_data.offset((self.cur - 1)
                as isize)))
        }
    }
}


pub struct StreamMut<'a> {
    context: &'a mut Context,
    index: i32,
    inner: Stream<'a>,
}

impl<'a> StreamMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: i32) -> StreamMut {
        StreamMut {
            context: mem::transmute_copy(&context),
            index,
            inner: Stream::wrap(mem::transmute_copy(&context), index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVStream {
        *(*self.context.as_mut_ptr()).streams.add(self.index as usize)
    }
}

impl<'a> StreamMut<'a> {
    pub fn set_time_base<R: Into<Rational>>(&mut self, time_base: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into().into();
        }
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = value.into().into();
        }
    }

    /// avg_frame_rate表示的是编码器在实际生成视频时使用的平均帧率
    ///
    /// 对于音频中：表示的是每秒钟平均的音频帧数，它的值可能与输入音频的采样率不同，因为编码器可能会对音频进行重采样
    /// 或者使用某些压缩技术来控制输出音频的码率。这样，实际输出音频的采样率可能会低于输入音频的采样率，
    /// 而 avg_frame_rate 就是反映这个实际采样率的参数。
    ///
    /// 在视频后期制作中，avg_frame_rate 参数非常有用。例如，在对视频进行加速、减速、调整帧率等操作时，需要知道
    /// 输入视频的帧率和输出视频的帧率，以便正确地进行处理
    ///
    /// 在音视频编码中:
    /// - demuxing （解封装）是将整个容器文件分离为独立的音频、视频或字幕流的过程。在 libavformat 库中，demuxing
    /// 是通过 AVFormatContext 结构体来实现的。AVFormatContext 结构体中包含了许多用于解封装的参数和状态信息，
    /// 例如容器格式、流的信息、解封装过程中的状态等。
    ///
    /// - muxing （多路复用）是将多个独立的音频、视频或字幕流合并为一个容器文件的过程。
    /// 在 libavformat 库中，muxing 是通过 AVFormatContext 结构体来实现的。在调用 avformat_write_header() 函数之前，可以通过设置 AVFormatContext 结构体中的一些参数来控制多路复用的行为，例如设置容器格式、编码器参数、流的时间基等。
    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = value.into().into();
        }
    }


    /// 用于存储流的编码器参数
    /// 通过libavformat中avformat_new_stream() 分配，以及avformat_free_context()进行释放。
    ///
    /// - demuxing（解封装）：在流创建的时候由libavformat或avformat_find_stream_info()方法来填充。
    /// - muxing（多路复用）：在调用avformat_write_header()方法之前填充
    pub fn set_codecpar<P: Into<Parameters>>(&mut self, value: P) {
        unsafe {
            ffi::avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, value.into().as_ptr());
        }
    }

    pub fn set_metadata(&mut self, value: Dictionary) {
        unsafe {
            let metadata = value;
            (*self.as_mut_ptr()).metadata = metadata.into_raw();
        }
    }
}

impl<'a> Deref for StreamMut<'a> {
    type Target = Stream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}




