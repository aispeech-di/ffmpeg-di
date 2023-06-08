use std::mem;
use std::ops::Deref;

use crate::ffi;
use crate::format::context::destroy;
use crate::format::stream::Stream;
use crate::format::input::InputFormat;
use crate::format::packet::Packet;

use super::Context;

pub struct InputContext {
    ptr: *mut ffi::AVFormatContext,
    ctx: Context,
}

impl InputContext {
    pub unsafe fn wrap(ptr: *mut ffi::AVFormatContext) -> Self {
        InputContext { ptr, ctx: Context::wrap(ptr, destroy::Mode::Input) }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVFormatContext {
        self.ptr as *const _
    }
    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFormatContext {
        self.ptr
    }
}

impl InputContext {
    // 输入格式上下文
    pub fn format(&self) -> InputFormat {
        unsafe { InputFormat::wrap((*self.as_ptr()).iformat as *mut ffi::AVInputFormat) }
    }

    // 输入包
    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }
}

impl Deref for InputContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut InputContext,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut InputContext) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (Stream<'a>, Packet);
    fn next(&mut self) -> Option<Self::Item> {
        let mut packet = Packet::empty();

        loop {
            match packet.read(self.context) {
                Ok(..) => unsafe {
                    return Some((Stream::wrap(mem::transmute_copy(&self.context), packet.stream_index()), packet));
                },
                Err(e) => {
                    if e.to_string() == "EOF".to_string() {
                        return None;
                    } else {
                        ()
                    }
                }
            }
        }
    }
}
