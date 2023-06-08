use bitflags::bitflags;
use crate::ffi::*;
use libc::c_uint;

bitflags! {
    pub struct Capabilities: c_uint {
        const DRAW_HORIZ_BAND           = AV_CODEC_CAP_DRAW_HORIZ_BAND;
        const DR1                       = AV_CODEC_CAP_DR1;
        const DELAY                     = AV_CODEC_CAP_DELAY;
        const SMALL_LAST_FRAME          = AV_CODEC_CAP_SMALL_LAST_FRAME;
        const SUBFRAMES                 = AV_CODEC_CAP_SUBFRAMES;
        const EXPERIMENTAL              = AV_CODEC_CAP_EXPERIMENTAL;
        const CHANNEL_CONF              = AV_CODEC_CAP_CHANNEL_CONF;
        const FRAME_THREADS             = AV_CODEC_CAP_FRAME_THREADS;
        const SLICE_THREADS             = AV_CODEC_CAP_SLICE_THREADS;
        const PARAM_CHANGE              = AV_CODEC_CAP_PARAM_CHANGE;
        const OTHER_THREADS              = AV_CODEC_CAP_OTHER_THREADS;
        const VARIABLE_FRAME_SIZE       = AV_CODEC_CAP_VARIABLE_FRAME_SIZE;
        const AVOID_PROBING             = AV_CODEC_CAP_AVOID_PROBING;
        const HARDWARE                  = AV_CODEC_CAP_HARDWARE;
        const HYBRID                    = AV_CODEC_CAP_HYBRID;
        const ENCODER_REORDERED_OPAQUE  = AV_CODEC_CAP_ENCODER_REORDERED_OPAQUE;
        const ENCODER_FLUSH             = AV_CODEC_CAP_ENCODER_FLUSH;
        const ENCODER_RECON_FRAME       = AV_CODEC_CAP_ENCODER_RECON_FRAME;
    }
}
