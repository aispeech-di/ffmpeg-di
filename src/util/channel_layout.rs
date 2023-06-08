use crate::ffi;

pub struct ChannelLayout {
    ptr: *mut ffi::AVChannelLayout,
}

impl ChannelLayout {
    pub unsafe fn wrap(ptr: *mut ffi::AVChannelLayout) -> Self {
        ChannelLayout { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVChannelLayout {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVChannelLayout {
        self.ptr
    }
}

impl ChannelLayout {
    pub fn default(nb_channels: i32) -> Self {
        let mut ch_layout = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::av_channel_layout_default(&mut ch_layout, nb_channels);
            ChannelLayout::wrap(&mut ch_layout)
        }
    }

    pub fn nb_channels(&self) -> i32 {
        unsafe { (*self.as_ptr()).nb_channels }
    }
}




