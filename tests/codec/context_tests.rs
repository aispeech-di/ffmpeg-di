use ffmpeg_di::codec::codec_id::avcodec_get_name;
use ffmpeg_di::codec::context::Context;
use ffmpeg_di::format::input::{find_input_format, open_with_format};
use ffmpeg_di::util::{frame, media};

#[test]
pub fn test_generated_codec_context() {
    let short_name = "s16le";
    let f = find_input_format(short_name).expect(&format!("no such input format: {}", short_name));
    let mut c = open_with_format("tests/assets/16bit_c1.pcm", f).unwrap();
    let s = c.streams().best(media::Type::Audio).unwrap();
    let p = s.parameters();
    let i = p.codec_id();
    let name = avcodec_get_name(i).unwrap();
    println!("{:?}", name);

    //codec_context
    let context = Context::parameters_to_context(p).unwrap();

    //获取decoder
    let mut audio = context.decoder().audio().unwrap();
    audio.set_rate(16000);
    let rate = audio.rate();
    let channels = audio.channels();

    for (_smp, pkt) in c.packets() {
        if pkt.stream() == 0 {
            audio.send_packet(&pkt).unwrap();

            let mut frame = frame::Frame::empty();
            audio.receive_packet(&mut frame).unwrap();
        }

    }
    println!("{} {}", rate, channels);

}