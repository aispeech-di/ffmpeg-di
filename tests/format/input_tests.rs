use ffmpeg_di::format::input::{find_input_format, open_with_format};
use ffmpeg_di::util::media::Type;

#[test]
fn test_av_find_input_format() {
    let f = find_input_format("wav").ok_or_else(|| "no such input format".to_string()).unwrap();
    println!("{:?}", f.name());
    println!("{:?}", f.long_name());

    for e in f.extensions() {
        println!("{:?}", e);
    }
    for m in f.mime_types() {
        println!("{:?}", m);
    }
}

#[test]
fn test_open_input_with_format() {
    let short_name = "s16le";
    let f = find_input_format(short_name).expect(&format!("no such input format: {}", short_name));
    let c = open_with_format("assets/16bit_c1.pcm", f);

    assert!(c.is_ok());
}

#[test]
fn test_best_stream() {
    let short_name = "s16le";
    let f = find_input_format(short_name).expect(&format!("no such input format: {}", short_name));
    let c = open_with_format("assets/16bit_c1.pcm", f);

    match c {
        Ok(v) => {
            let s = v.streams().best(Type::Audio).ok_or_else(|| "no best stream".to_string())
                .unwrap();

            println!("stream: {:?}", s.duration());
        }
        Err(_) => {}
    }
}