use crate::ffi;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input,
    Output,
}

pub struct Destroy {
    ptr: *mut ffi::AVFormatContext,
    mode: Mode,
}

impl Destroy {
    pub fn new(ptr: *mut ffi::AVFormatContext, mode: Mode) -> Self {
        Self { ptr, mode }
    }
}

impl Drop for Destroy {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => ffi::avformat_close_input(&mut self.ptr),
                Mode::Output => {
                    ffi::avio_close((*self.ptr).pb);
                    ffi::avformat_free_context(self.ptr);
                }
            }
        }
    }
}