use crate::ffi;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<ffi::AVMediaType> for Type {
    fn from(value: ffi::AVMediaType) -> Self {
        match value {
            ffi::AVMediaType::AVMEDIA_TYPE_UNKNOWN => Type::Unknown,
            ffi::AVMediaType::AVMEDIA_TYPE_VIDEO => Type::Video,
            ffi::AVMediaType::AVMEDIA_TYPE_AUDIO => Type::Audio,
            ffi::AVMediaType::AVMEDIA_TYPE_DATA => Type::Data,
            ffi::AVMediaType::AVMEDIA_TYPE_SUBTITLE => Type::Subtitle,
            ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT => Type::Attachment,
            ffi::AVMediaType::AVMEDIA_TYPE_NB => Type::Unknown,
        }
    }
}

impl From<Type> for ffi::AVMediaType {
    #[inline(always)]
    fn from(value: Type) -> ffi::AVMediaType {
        match value {
            Type::Unknown => ffi::AVMediaType::AVMEDIA_TYPE_UNKNOWN,
            Type::Video => ffi::AVMediaType::AVMEDIA_TYPE_VIDEO,
            Type::Audio => ffi::AVMediaType::AVMEDIA_TYPE_AUDIO,
            Type::Data => ffi::AVMediaType::AVMEDIA_TYPE_DATA,
            Type::Subtitle => ffi::AVMediaType::AVMEDIA_TYPE_SUBTITLE,
            Type::Attachment => ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT,
        }
    }
}