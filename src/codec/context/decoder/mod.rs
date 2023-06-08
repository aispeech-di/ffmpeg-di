pub mod audio;


pub use self::audio::Audio;
use std::ops::{Deref, DerefMut};
use std::ptr;
use super::Context;
use anyhow::{anyhow, Result};
use crate::codec::codec::Codec;
use crate::ffi;
use crate::format::packet::Packet;
use crate::util::dict::Dictionary;
use crate::util::frame::Frame;

pub struct Decoder {
    pub context: Context,
    pub open: bool,
}

impl Decoder {
    pub fn open(mut self) -> Result<Decoder> {
        unsafe {
            match ffi::avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => {
                    self.open = true;
                    Ok(self)
                }
                e => Err(anyhow!("avcodec open failed: {}", e)),
            }
        }
    }

    pub fn open_as(mut self, codec: Codec) -> Result<Decoder> {
        unsafe {
            match ffi::avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                0 => {
                    self.open = true;
                    Ok(self)
                }
                e => Err(anyhow!("avcodec open failed: {}", e)),
            }
        }
    }

    pub fn open_as_with(mut self, codec: Codec, opt: Dictionary) -> Result<Decoder> {
        unsafe {
            let mut dict = opt.into_raw();
            let res = ffi::avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut dict);
            Dictionary::from_raw(dict);
            match res {
                0 => {
                    self.open = true;
                    Ok(self)
                }
                e => Err(anyhow!("avcodec open failed: {}", e)),
            }
        }
    }

    pub fn audio(self) -> Result<Audio> {
        if let Some(codec) = super::find(self.id()) {
            let opened = self.open_as(codec);
            match opened {
                Ok(audio) => Ok(Audio(audio)),
                Err(e) => Err(anyhow!("avcodec open failed: {}", e)),
            }
        } else {
            return Err(anyhow!("audio decoder not found"));
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        unsafe {
            if self.open {
                match ffi::avcodec_send_packet(self.as_mut_ptr(), packet.as_ptr()) {
                    e if e < 0 => Err(anyhow!("send packet failed: {}", e)),
                    _ => Ok(()),
                }
            } else {
                Err(anyhow!("decoder not open"))
            }

        }
    }

    /// 结束解码，进入排水模式
    pub fn send_eof(&mut self) -> Result<()> {
        unsafe {
            if self.open {
                match ffi::avcodec_send_packet(self.as_mut_ptr(), ptr::null()) {
                    e if e < 0 => Err(anyhow!("send eof failed: {}", e)),
                    _ => Ok(()),
                }
            } else {
                Err(anyhow!("decoder not open"))
            }

        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) ->Result<()> {
        unsafe {
            if self.open {
                match ffi::avcodec_receive_frame(self.as_mut_ptr(), frame.as_mut_ptr()) {
                    e if e < 0 => Err(anyhow!("receive frame failed: {}", e)),
                    _ => Ok(()),
                }
            } else {
                Err(anyhow!("decoder not open"))
            }

        }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl AsRef<Context> for Decoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Decoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}