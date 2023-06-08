pub mod input;
pub mod destroy;

use std::{fmt, ptr};
use std::fmt::Formatter;
use std::rc::Rc;
use crate::ffi;
use crate::util::media;

use self::destroy::Destroy;
use crate::format::stream::Stream;


pub struct Context {
    ptr: *mut ffi::AVFormatContext,
    dtor: Rc<Destroy>,
}

impl Context {
    pub unsafe fn wrap(ptr: *mut ffi::AVFormatContext, mode: destroy::Mode) -> Self {
        Self { ptr, dtor: Rc::new(Destroy::new(ptr, mode)) }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFormatContext {
        self.ptr
    }

    pub unsafe fn destroy(&self) -> Rc<Destroy> {
        Rc::clone(&self.dtor)
    }
}

impl Context {
    pub fn nb_streams(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_streams }
    }

    pub fn streams(&self) -> StreamIter {
        StreamIter::new(self)
    }

    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.as_ptr()).bit_rate }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("AVFormatContext");
        s.field("bit_rate", &self.bit_rate());
        s.field("duration", &self.duration());
        s.field("nb_streams", &self.nb_streams());
        s.finish()
    }
}

#[derive(Debug)]
pub struct StreamIter<'a> {
    context: &'a Context,
}

impl<'a> StreamIter<'a> {
    pub fn new(context: &'a Context) -> StreamIter<'a> {
        StreamIter {
            context
        }
    }
}

impl<'a> StreamIter<'a> {
    pub fn best<'b>(&self, kind: media::Type) -> Option<Stream<'b>>
        where 'a: 'b
    {
        unsafe {
            let decoder = ptr::null_mut();
            let index = ffi::av_find_best_stream(
                self.context.ptr,
                kind.into(),
                -1,
                -1,
                decoder,
                0,
            );

            if index >= 0 {
                Some(Stream::wrap(self.context, index))
            } else {
                None
            }
        }
    }
}

