pub mod decoder;

use std::any::Any;
use std::ptr;
use std::rc::Rc;
use crate::codec::codec_par::Parameters;
use crate::ffi;
use anyhow::{anyhow, Result};
use crate::codec::codec::Codec;
use crate::codec::codec_id::CodecId;
use crate::codec::context::decoder::Decoder;

pub struct Context {
    ptr: *mut ffi::AVCodecContext,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut ffi::AVCodecContext, owner: Option<Rc<dyn Any>>) -> Self {
        Context { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVCodecContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVCodecContext {
        self.ptr
    }
}

impl Context {
    /// 创建一个空的编码器上下文
    pub fn new() -> Self {
        unsafe {
            Context { ptr: ffi::avcodec_alloc_context3(ptr::null()), owner: None }
        }
    }

    /// 创建编码器上下文，将编码器参数拷贝到上下文中
    pub fn parameters_to_context<P: Into<Parameters>>(parameters: P) -> Result<Self> {
        let parameters = parameters.into();
        let mut context = Self::new();

        unsafe {
            match ffi::avcodec_parameters_to_context(context.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(anyhow!("codec parameters copy to context failed: {}", e)),
                _ => Ok(context),
            }
        }
    }

    /// 通过编解码器编号创建编码器上下文
    pub fn new_with_codec(codec: &Codec) -> Self {
        unsafe {
            Context { ptr: ffi::avcodec_alloc_context3(codec.as_ptr()), owner: None }
        }
    }

    /// 编解码器编号
    pub fn id(&self) -> CodecId {
        unsafe {
            CodecId::from((*self.as_ptr()).codec_id)
        }
    }

    /// 解码器
    pub fn decoder(self) -> Decoder {
        Decoder {
            context: self,
            open: false,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                ffi::avcodec_free_context(&mut self.as_mut_ptr());
            }
        }
    }
}

/// 通过编解码器ID获取编解码对象
pub fn find(id: CodecId) -> Option<Codec> {
    unsafe {
        let ptr = ffi::avcodec_find_decoder(id.into()) as *mut ffi::AVCodec;
        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}