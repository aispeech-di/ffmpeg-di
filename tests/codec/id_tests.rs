// tests/codec/id_tests.rs

use ffmpeg_di::codec::codec_id::{avcodec_get_name, CodecId};

#[test]
pub fn test_avcodec_get_name() {
    let name = avcodec_get_name(CodecId::PCM_S16BE);
    println!("{:?}", name.unwrap());
}