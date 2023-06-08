pub mod side_data;
pub mod side_data_type;

use anyhow::{anyhow, Result};
use libc::c_int;

use crate::ffi;
use crate::format::context::input::InputContext;

/// 音频数据包结构
pub struct Packet {
    ptr: *mut ffi::AVPacket,
}

impl Packet {
    pub unsafe fn wrap(ptr: *mut ffi::AVPacket) -> Self {
        Packet { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVPacket {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVPacket {
        self.ptr
    }
}

impl Packet {
    #[inline]
    pub fn empty() -> Self {
        unsafe {
            let pkt = ffi::av_packet_alloc();
            Packet::wrap(pkt)
        }
    }

    #[inline]
    pub fn new(size: i32) -> Self {
        unsafe {
            let mut pkt = Self::empty();
            ffi::av_new_packet(pkt.as_mut_ptr(), size as c_int);
            pkt
        }
    }

    #[inline]
    pub fn size(&self) -> i32 {
        unsafe {
            (*self.as_ptr()).size
        }
    }

    #[inline]
    pub fn stream_index(&self) -> i32 {
        unsafe {
            (*self.as_ptr()).stream_index
        }
    }

    #[inline]
    pub fn read(&mut self, format: &mut InputContext) -> Result<()> {
        unsafe {
            match ffi::av_read_frame(format.as_mut_ptr(), self.as_mut_ptr()) {
                0 => Ok(()),
                e if e == ffi::AVERROR_EOF => Err(anyhow!("EOF")),
                e => Err(anyhow!("{}", e)),
            }
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            ffi::av_packet_unref(self.as_mut_ptr());
        }
    }
}



