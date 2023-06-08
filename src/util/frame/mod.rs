pub mod audio;
pub mod side_data;
pub use self::audio::Audio;

use crate::ffi;
use crate::util::dict::DictRef;

pub struct Frame {
    ptr: *mut ffi::AVFrame,
}


impl Frame {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut ffi::AVFrame) -> Self {
        Frame { ptr }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const ffi::AVFrame {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFrame {
        self.ptr
    }
}

impl Frame {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe {
            Frame {
                ptr: ffi::av_frame_alloc()
            }
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        unsafe {
            (*self.as_ptr()).data[0].is_null()
        }
    }

    #[inline]
    pub fn duration(&self) -> i64 {
        unsafe {
            (*self.as_ptr()).duration
        }
    }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        unsafe {
            match (*self.as_ptr()).pts {
                ffi::AV_NOPTS_VALUE => None,
                pts => Some(pts)
            }
        }
    }

    #[inline]
    pub fn metadata(&self) -> DictRef {
        unsafe {
            DictRef::wrap((*self.as_ptr()).metadata)
        }
    }

    #[inline]
    pub fn best_effort_timestamp(&self) -> i64 {
        unsafe {
            (*self.as_ptr()).best_effort_timestamp
        }
    }
}

impl Drop for Frame {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::av_frame_free(&mut self.as_mut_ptr());
        }
    }
}