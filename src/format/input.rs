use std::ffi::{CStr, CString};
use std::ptr;
use std::str::from_utf8_unchecked;

use anyhow::{anyhow, Result};

use crate::ffi;
use crate::format::context::input::InputContext;

pub struct InputFormat {
    ptr: *mut ffi::AVInputFormat,
}

impl InputFormat {
    pub unsafe fn wrap(ptr: *mut ffi::AVInputFormat) -> Self {
        InputFormat { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVInputFormat {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVInputFormat {
        self.ptr
    }
}

impl InputFormat {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn long_name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
    }

    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;
            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }
    pub fn mime_types(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }
}

/// 根据输入格式的简称查找 AVInputFormat
pub fn find_input_format(name: &str) -> Option<InputFormat> {
    unsafe {
        let c_name = CString::new(name).unwrap();
        let ptr = ffi::av_find_input_format(c_name.as_ptr() as *const libc::c_char) as *mut ffi::AVInputFormat;
        if ptr.is_null() {
            None
        } else {
            Some(InputFormat::wrap(ptr))
        }
    }
}

/// 打开音频文件，自动推算input format
pub fn open(filename: &str) -> Result<InputContext> {
    open_input(filename, ptr::null_mut())
}

/// 打开一个输入流并读取标题。解码器没有打开。流必须用 avformat_close_input()关闭。
pub fn open_with_format(filename: &str, input_format: InputFormat) -> Result<InputContext> {
    unsafe {
        open_input(filename, input_format.as_ptr())
    }
}

fn open_input(filename: &str, input_format: *const ffi::AVInputFormat) -> Result<InputContext> {
    unsafe {
        let mut ctx = ptr::null_mut();
        let c_filename = CString::new(filename).unwrap();

        match ffi::avformat_open_input(&mut ctx, c_filename.as_ptr(), input_format, ptr::null_mut()) {
            0 => {
                match ffi::avformat_find_stream_info(ctx, ptr::null_mut()) {
                    r if r >= 0 => Ok(InputContext::wrap(ctx)),
                    e => {
                        ffi::avformat_close_input(&mut ctx);
                        Err(anyhow!("find stream info failed: {}", e))
                    }
                }
            }
            e => Err(anyhow!("open failed: {}", e))
        }
    }
}

