use bitflags::bitflags;
use crate::ffi;
use libc::c_int;

bitflags! {
    pub struct Disposition: c_int {
        const DEFAULT          = ffi::AV_DISPOSITION_DEFAULT;
        const DUB              = ffi::AV_DISPOSITION_DUB;
        const ORIGINAL         = ffi::AV_DISPOSITION_ORIGINAL;
        const COMMENT          = ffi::AV_DISPOSITION_COMMENT;
        const LYRICS           = ffi::AV_DISPOSITION_LYRICS;
        const KARAOKE          = ffi::AV_DISPOSITION_KARAOKE;
        const FORCED           = ffi::AV_DISPOSITION_FORCED;
        const HEARING_IMPAIRED = ffi::AV_DISPOSITION_HEARING_IMPAIRED;
        const VISUAL_IMPAIRED  = ffi::AV_DISPOSITION_VISUAL_IMPAIRED;
        const CLEAN_EFFECTS    = ffi::AV_DISPOSITION_CLEAN_EFFECTS;
        const ATTACHED_PIC     = ffi::AV_DISPOSITION_ATTACHED_PIC;
        const CAPTIONS         = ffi::AV_DISPOSITION_CAPTIONS;
        const DESCRIPTIONS     = ffi::AV_DISPOSITION_DESCRIPTIONS;
        const METADATA         = ffi::AV_DISPOSITION_METADATA;
    }
}