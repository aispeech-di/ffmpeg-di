use crate::ffi;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Discard {
    None,
    Default,
    NonReference,
    Bidirectional,
    NonIntra,
    NonKey,
    All,
}

impl From<ffi::AVDiscard> for Discard {
    fn from(value: ffi::AVDiscard) -> Self {
        match value {
            ffi::AVDiscard::AVDISCARD_NONE => Discard::None,
            ffi::AVDiscard::AVDISCARD_DEFAULT => Discard::Default,
            ffi::AVDiscard::AVDISCARD_NONREF => Discard::NonReference,
            ffi::AVDiscard::AVDISCARD_BIDIR => Discard::Bidirectional,
            ffi::AVDiscard::AVDISCARD_NONINTRA => Discard::NonIntra,
            ffi::AVDiscard::AVDISCARD_NONKEY => Discard::NonKey,
            ffi::AVDiscard::AVDISCARD_ALL => Discard::All,
        }
    }
}

impl From<Discard> for ffi::AVDiscard {
    fn from(value: Discard) -> ffi::AVDiscard {
        match value {
            Discard::None =>  ffi::AVDiscard::AVDISCARD_NONE,
            Discard::Default =>  ffi::AVDiscard::AVDISCARD_DEFAULT,
            Discard::NonReference =>  ffi::AVDiscard::AVDISCARD_NONREF,
            Discard::Bidirectional =>  ffi::AVDiscard::AVDISCARD_BIDIR,
            Discard::NonIntra =>  ffi::AVDiscard::AVDISCARD_NONINTRA,
            Discard::NonKey =>  ffi::AVDiscard::AVDISCARD_NONKEY,
            Discard::All =>  ffi::AVDiscard::AVDISCARD_ALL,
        }
    }
}