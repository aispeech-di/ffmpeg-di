use std::any::Any;
use std::rc::Rc;

use crate::codec::codec_id::CodecId;
use crate::ffi;

pub struct Parameters {
    ptr: *mut ffi::AVCodecParameters,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    pub unsafe fn wrap(ptr: *mut ffi::AVCodecParameters, owner: Option<Rc<dyn Any>>) -> Self {
        Parameters { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVCodecParameters {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVCodecParameters {
        self.ptr
    }
}

impl Parameters {
    pub fn new() -> Self {
        unsafe {
            Parameters { ptr: ffi::avcodec_parameters_alloc(), owner: None }
        }
    }

    #[inline]
    pub fn codec_id(&self) ->CodecId {
        unsafe { (*self.as_ptr()).codec_id.into() }
    }
}

impl Drop for Parameters {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                ffi::avcodec_parameters_free(&mut self.as_mut_ptr());
            }
        }
    }
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        let mut par = Parameters::new();
        unsafe {
            ffi::avcodec_parameters_copy(par.as_mut_ptr(), self.as_ptr());
        }
        par

    }
}