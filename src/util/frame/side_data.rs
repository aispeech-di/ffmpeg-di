use std::ffi::CStr;
use std::marker::PhantomData;
use std::slice;
use std::str::from_utf8_unchecked;
use crate::ffi;
use crate::util::dict::DictRef;
use crate::util::frame::Frame;

pub struct FrameSideData<'a> {
    ptr: *mut ffi::AVFrameSideData,
    _marker: PhantomData<&'a Frame>,
}


impl<'a> FrameSideData<'a> {
    #[inline]
    pub unsafe fn wrap(ptr: *mut ffi::AVFrameSideData) -> Self {
        FrameSideData { ptr, _marker: PhantomData, }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const ffi::AVFrameSideData {
        self.ptr as *const _
    }

    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFrameSideData {
        self.ptr
    }
}

impl<'a> FrameSideData<'a> {

    #[inline]
    pub fn kind(&self) -> FrameSideDataType {
        unsafe { FrameSideDataType::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size)
        }
    }

    pub fn metadata(&self) -> DictRef {
        unsafe {
            DictRef::wrap((*self.as_ptr()).metadata)
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FrameSideDataType {
    PANSCAN,
    A53_CC,
    STEREO3D,
    MATRIXENCODING,
    DOWNMIX_INFO,
    REPLAYGAIN,
    DISPLAYMATRIX,
    AFD,
    MOTION_VECTORS,
    SKIP_SAMPLES,
    AUDIO_SERVICE_TYPE,
    MASTERING_DISPLAY_METADATA,
    GOP_TIMECODE,
    SPHERICAL,
    CONTENT_LIGHT_LEVEL,
    ICC_PROFILE,
    S12M_TIMECODE,
    DYNAMIC_HDR_PLUS,
    REGIONS_OF_INTEREST,
    VIDEO_ENC_PARAMS,
    SEI_UNREGISTERED,
    FILM_GRAIN_PARAMS,
    DETECTION_BBOXES,
    DOVI_RPU_BUFFER,
    DOVI_METADATA,
    DYNAMIC_HDR_VIVID,
    AMBIENT_VIEWING_ENVIRONMENT,
}

impl From<ffi::AVFrameSideDataType> for FrameSideDataType {
    fn from(value: ffi::AVFrameSideDataType) -> Self {
        match value {
            ffi::AVFrameSideDataType::AV_FRAME_DATA_PANSCAN => FrameSideDataType::PANSCAN,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_A53_CC => FrameSideDataType::A53_CC,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_STEREO3D => FrameSideDataType::STEREO3D,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_MATRIXENCODING => FrameSideDataType::MATRIXENCODING,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DOWNMIX_INFO => FrameSideDataType::DOWNMIX_INFO,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_REPLAYGAIN => FrameSideDataType::REPLAYGAIN,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DISPLAYMATRIX => FrameSideDataType::DISPLAYMATRIX,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_AFD => FrameSideDataType::AFD,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_MOTION_VECTORS => FrameSideDataType::MOTION_VECTORS,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_SKIP_SAMPLES => FrameSideDataType::SKIP_SAMPLES,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_AUDIO_SERVICE_TYPE => FrameSideDataType::AUDIO_SERVICE_TYPE,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => FrameSideDataType::MASTERING_DISPLAY_METADATA,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_GOP_TIMECODE => FrameSideDataType::GOP_TIMECODE,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_SPHERICAL => FrameSideDataType::SPHERICAL,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => FrameSideDataType::CONTENT_LIGHT_LEVEL,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_ICC_PROFILE => FrameSideDataType::ICC_PROFILE,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_S12M_TIMECODE => FrameSideDataType::S12M_TIMECODE,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DYNAMIC_HDR_PLUS => FrameSideDataType::DYNAMIC_HDR_PLUS,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_REGIONS_OF_INTEREST => FrameSideDataType::REGIONS_OF_INTEREST,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_VIDEO_ENC_PARAMS => FrameSideDataType::VIDEO_ENC_PARAMS,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_SEI_UNREGISTERED => FrameSideDataType::SEI_UNREGISTERED,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_FILM_GRAIN_PARAMS => FrameSideDataType::FILM_GRAIN_PARAMS,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DETECTION_BBOXES => FrameSideDataType::DETECTION_BBOXES,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DOVI_RPU_BUFFER => FrameSideDataType::DOVI_RPU_BUFFER,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DOVI_METADATA => FrameSideDataType::DOVI_METADATA,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_DYNAMIC_HDR_VIVID => FrameSideDataType::DYNAMIC_HDR_VIVID,
            ffi::AVFrameSideDataType::AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT => FrameSideDataType::AMBIENT_VIEWING_ENVIRONMENT
        }
    }
}

impl From<FrameSideDataType> for ffi::AVFrameSideDataType {
    fn from(value: FrameSideDataType) -> Self {
        match value {
            FrameSideDataType::PANSCAN => ffi::AVFrameSideDataType::AV_FRAME_DATA_PANSCAN,
            FrameSideDataType::A53_CC => ffi::AVFrameSideDataType::AV_FRAME_DATA_A53_CC,
            FrameSideDataType::STEREO3D => ffi::AVFrameSideDataType::AV_FRAME_DATA_STEREO3D,
            FrameSideDataType::MATRIXENCODING => ffi::AVFrameSideDataType::AV_FRAME_DATA_MATRIXENCODING,
            FrameSideDataType::DOWNMIX_INFO => ffi::AVFrameSideDataType::AV_FRAME_DATA_DOWNMIX_INFO,
            FrameSideDataType::REPLAYGAIN => ffi::AVFrameSideDataType::AV_FRAME_DATA_REPLAYGAIN,
            FrameSideDataType::DISPLAYMATRIX => ffi::AVFrameSideDataType::AV_FRAME_DATA_DISPLAYMATRIX,
            FrameSideDataType::AFD => ffi::AVFrameSideDataType::AV_FRAME_DATA_AFD,
            FrameSideDataType::MOTION_VECTORS => ffi::AVFrameSideDataType::AV_FRAME_DATA_MOTION_VECTORS,
            FrameSideDataType::SKIP_SAMPLES => ffi::AVFrameSideDataType::AV_FRAME_DATA_SKIP_SAMPLES,
            FrameSideDataType::AUDIO_SERVICE_TYPE => ffi::AVFrameSideDataType::AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
            FrameSideDataType::MASTERING_DISPLAY_METADATA => ffi::AVFrameSideDataType::AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            FrameSideDataType::GOP_TIMECODE => ffi::AVFrameSideDataType::AV_FRAME_DATA_GOP_TIMECODE,
            FrameSideDataType::SPHERICAL => ffi::AVFrameSideDataType::AV_FRAME_DATA_SPHERICAL,
            FrameSideDataType::CONTENT_LIGHT_LEVEL => ffi::AVFrameSideDataType::AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            FrameSideDataType::ICC_PROFILE => ffi::AVFrameSideDataType::AV_FRAME_DATA_ICC_PROFILE,
            FrameSideDataType::S12M_TIMECODE => ffi::AVFrameSideDataType::AV_FRAME_DATA_S12M_TIMECODE,
            FrameSideDataType::DYNAMIC_HDR_PLUS => ffi::AVFrameSideDataType::AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
            FrameSideDataType::REGIONS_OF_INTEREST => ffi::AVFrameSideDataType::AV_FRAME_DATA_REGIONS_OF_INTEREST,
            FrameSideDataType::VIDEO_ENC_PARAMS => ffi::AVFrameSideDataType::AV_FRAME_DATA_VIDEO_ENC_PARAMS,
            FrameSideDataType::SEI_UNREGISTERED => ffi::AVFrameSideDataType::AV_FRAME_DATA_SEI_UNREGISTERED,
            FrameSideDataType::FILM_GRAIN_PARAMS => ffi::AVFrameSideDataType::AV_FRAME_DATA_FILM_GRAIN_PARAMS,
            FrameSideDataType::DETECTION_BBOXES => ffi::AVFrameSideDataType::AV_FRAME_DATA_DETECTION_BBOXES,
            FrameSideDataType::DOVI_RPU_BUFFER => ffi::AVFrameSideDataType::AV_FRAME_DATA_DOVI_RPU_BUFFER,
            FrameSideDataType::DOVI_METADATA => ffi::AVFrameSideDataType::AV_FRAME_DATA_DOVI_METADATA,
            FrameSideDataType::DYNAMIC_HDR_VIVID => ffi::AVFrameSideDataType::AV_FRAME_DATA_DYNAMIC_HDR_VIVID,
            FrameSideDataType::AMBIENT_VIEWING_ENVIRONMENT => ffi::AVFrameSideDataType::AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT,
        }
    }
}

impl FrameSideDataType {
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(ffi::av_frame_side_data_name((*self).into())).to_bytes())
        }
    }
}