use std::marker::PhantomData;
use std::slice;
use crate::ffi;
use crate::format::packet::Packet;
use crate::format::packet::side_data_type::PacketSideDataType;

pub struct PacketSideData<'a> {
    ptr: *mut ffi::AVPacketSideData,
    _marker: PhantomData<&'a Packet>,
}

impl<'a> PacketSideData<'a> {
    pub unsafe fn wrap(ptr: *mut ffi::AVPacketSideData) -> Self {
        PacketSideData { ptr, _marker: PhantomData }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVPacketSideData {
        self.ptr as *const _
    }
}

impl<'a> PacketSideData<'a> {
    pub fn kind(&self) -> PacketSideDataType {
        unsafe {
            PacketSideDataType::from((*self.as_ptr()).type_)
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size)
        }
    }
}