use ffmpeg_di::codec::context::{Context, decoder};
use ffmpeg_di::format::input::open;
use ffmpeg_di::format::packet::Packet;
use ffmpeg_di::util::{media};
use anyhow::{anyhow, Result};

fn main() {
    // 音频输入上下文
    //let mut c = open("examples/assets/样本1.wav").unwrap();
    //let mut c = open("examples/assets/snd_u8.wav").unwrap();
    let mut c = open("examples/assets/c2_sample.wav").unwrap();
    let input = c.format();
    println!("name: {:?}", input.name());
    println!("long_name: {:?}", input.long_name());
    println!("extensions: {:?}", input.extensions());
    println!("mime_types: {:?}", input.mime_types());

    let stream = c.streams().best(media::Type::Audio).unwrap();
    let d = c.duration();
    let nb_streams = c.nb_streams();
    println!("input context duration: {:?}", d);

    let parameters = stream.parameters();
    let codec_id = parameters.codec_id();
    println!("codec id name: {:?}", codec_id.name());
    println!("codec id type: {:?}", codec_id.medium());


    let index = stream.index();
    let duration = stream.duration();
    let time_base = stream.time_base();
    let id = stream.id();
    // 返回秒
    let ddd: f64 = duration as f64 * f64::from(stream.time_base());
    let start_time = stream.start_time();
    let nb_frames = stream.nb_frames();
    let disposition = stream.disposition();
    let discard = stream.discard();
    let _side_data = stream.side_data();

    println!("stream duration: {:?}", duration);
    println!("nb_streams: {:?}", nb_streams);
    println!("time_base: {:?}", time_base);
    println!("ddd: {:?}", ddd);
    println!("stream index: {:?}", index);
    println!("stream id: {:?}", id);
    println!("start_time: {:?}", start_time);
    println!("nb_frames: {:?}", nb_frames);
    println!("disposition: {:?}", disposition.bits());
    println!("discard: {:?}", discard);


    //解码器上下文
    let codec_ctx = Context::parameters_to_context(parameters).unwrap();
    //音频解码器
    let mut audio_decoder = codec_ctx.decoder().audio().unwrap();
    let sample_fmt = audio_decoder.format();
    let is_planar = sample_fmt.is_planar();
    let bytes = sample_fmt.bytes();

    println!("sample format: {:?}", sample_fmt.name());
    println!("sample format is planar: {:?}", is_planar);
    println!("sample bytes: {:?}", bytes);

    for (stm, pkt) in c.packets() {
        if stm.index() == index {
            send_packet_to_decoder(&mut audio_decoder, &pkt).unwrap();

            // receive_and_process_decoded_frames(&mut audio_decoder);
        }
    }
}

fn send_packet_to_decoder(decoder: &mut decoder::Audio, packet: &Packet) -> Result<()> {
    match decoder.send_packet(packet) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("发送音频包到解码器失败：{}", e))
    }
}

// fn receive_and_process_decoded_frames(decoder: &mut decoder::Audio) -> &[u8] {
//     let mut audio_frame = frame::Audio::empty();
//     while decoder.receive_frame(&mut audio_frame).is_ok() {
//         let samplefmt = audio_frame.format();
//         let num_channels = audio_frame.channels();
//         let time = audio_frame.duration();
//         let pts = audio_frame.pts();
//         let best_effort_timestamp = audio_frame.best_effort_timestamp();
//         let samples = audio_frame.samples();
//         println!("sample duration: {:?}", time);
//         println!("pts: {:?}", pts.unwrap());
//         println!("best_effort_timestamp: {:?}", best_effort_timestamp);
//         match (samplefmt, num_channels) {
//             (SampleFormat::NONE, _) => [0;1],
//             (SampleFormat::U8 | SampleFormat::U8P, 1) => audio_frame.data::<u8>(0),
//             (SampleFormat::U8, 2) => audio_frame.data::<(u8, u8)>(0),
//             (SampleFormat::U8, 3) => audio_frame.data::<(u8, u8, u8)>(0),
//             (SampleFormat::U8, 4) => audio_frame.data::<(u8, u8, u8, u8)>(0),
//             (SampleFormat::U8, 5) => audio_frame.data::<(u8, u8, u8, u8, u8)>(0),
//             (SampleFormat::U8, 6) => audio_frame.data::<(u8, u8, u8, u8, u8, u8)>(0),
//             (SampleFormat::U8, 7) => audio_frame.data::<(u8, u8, u8, u8, u8, u8, u8)>(0),
//             (SampleFormat::S16 | SampleFormat::S16P, 1) => audio_frame.data::<i16>(0),
//             (SampleFormat::S16, 2) => audio_frame.data::<(i16, i16)>(0),
//             (SampleFormat::S16, 3) => audio_frame.data::<(i16, i16, i16)>(0),
//             (SampleFormat::S16, 4) => audio_frame.data::<(i16, i16, i16, i16)>(0),
//             (SampleFormat::S16, 5) => audio_frame.data::<(i16, i16, i16, i16, i16)>(0),
//             (SampleFormat::S16, 6) => audio_frame.data::<(i16, i16, i16, i16, i16, i16)>(0),
//             (SampleFormat::S16, 7) => audio_frame.data::<(i16, i16, i16, i16, i16, i16, i16)>(0),
//             (SampleFormat::S32, 1) => audio_frame.data::<i32>(0),
//             (SampleFormat::S32, 2) => audio_frame.data::<(i32, i32)>(0),
//             (SampleFormat::S32, 3) => audio_frame.data::<(i32, i32, i32)>(0),
//             (SampleFormat::S32, 4) => audio_frame.data::<(i32, i32, i32, i32)>(0),
//             (SampleFormat::S32, 5) => audio_frame.data::<(i32, i32, i32, i32, i32)>(0),
//             (SampleFormat::S32, 6) => audio_frame.data::<(i32, i32, i32, i32, i32, i32)>(0),
//             (SampleFormat::S32, 7) => audio_frame.data::<(i32, i32, i32, i32, i32, i32, i32)>(0),
//             (SampleFormat::S64, 1) => audio_frame.data::<i64>(0),
//             (SampleFormat::S64, 2) => audio_frame.data::<(i64, i64)>(0),
//             (SampleFormat::S64, 3) => audio_frame.data::<(i64, i64, i64)>(0),
//             (SampleFormat::S64, 4) => audio_frame.data::<(i64, i64, i64, i64)>(0),
//             (SampleFormat::S64, 5) => audio_frame.data::<(i64, i64, i64, i64, i64)>(0),
//             (SampleFormat::S64, 6) => audio_frame.data::<(i64, i64, i64, i64, i64, i64)>(0),
//             (SampleFormat::S64, 7) => audio_frame.data::<(i64, i64, i64, i64, i64, i64, i64)>(0),
//             (SampleFormat::FLT, 1) => audio_frame.data::<f32>(0),
//             (SampleFormat::FLT, 2) => audio_frame.data::<(f32, f32)>(0),
//             (SampleFormat::FLT, 3) => audio_frame.data::<(f32, f32, f32)>(0),
//             (SampleFormat::FLT, 4) => audio_frame.data::<(f32, f32, f32, f32)>(0),
//             (SampleFormat::FLT, 5) => audio_frame.data::<(f32, f32, f32, f32, f32)>(0),
//             (SampleFormat::FLT, 6) => audio_frame.data::<(f32, f32, f32, f32, f32, f32)>(0),
//             (SampleFormat::FLT, 7) => audio_frame.data::<(f32, f32, f32, f32, f32, f32, f32)>(0),
//             (SampleFormat::DBL, 1) => audio_frame.data::<f64>(0),
//             (SampleFormat::DBL, 2) => audio_frame.data::<(f64, f64)>(0),
//             (SampleFormat::DBL, 3) => audio_frame.data::<(f64, f64, f64)>(0),
//             (SampleFormat::DBL, 4) => audio_frame.data::<(f64, f64, f64, f64)>(0),
//             (SampleFormat::DBL, 5) => audio_frame.data::<(f64, f64, f64, f64, f64)>(0),
//             (SampleFormat::DBL, 6) => audio_frame.data::<(f64, f64, f64, f64, f64, f64)>(0),
//             (SampleFormat::DBL, 7) => audio_frame.data::<(f64, f64, f64, f64, f64, f64, f64)>(0),
//
//             (SampleFormat::U8P, num_channels) => {
//                 let mut d = Vec::with_capacity(samples as usize);
//                 let mut sample_data = vec![0u8; num_channels as usize];
//                 for sample in 0..samples {
//                     for channel in 0..num_channels {
//                         sample_data[channel as usize] = audio_frame.data::<u8>(channel)[sample as usize];
//                     }
//                     let sample_tuple = match num_channels {
//                         2 => (sample_data[0], sample_data[1]),
//                         3 => (sample_data[0], sample_data[1], sample_data[2]),
//                         4 => (sample_data[0], sample_data[1], sample_data[2], sample_data[3]),
//                         5 => (sample_data[0], sample_data[1], sample_data[2], sample_data[3], sample_data[4]),
//                         _ => unreachable!(),
//                     };
//                     d.push(sample_tuple);
//                 }
//                 d.as_slice()
//             }
//
//
//             _ => {}
//         }
//
//         //     SampleFormat::S16 => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<i16>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(i16, i16)>(0);
//         //                 let samples: Vec<(i16, i16)> = data.iter().take(50).cloned().collect();
//         //                 println!("{:?}", samples);
//         //                 println!("data length: {:?}", data.len());
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(i16, i16, i16)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(i16, i16, i16, i16)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(i16, i16, i16, i16, i16)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(i16, i16, i16, i16, i16, i16)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(i16, i16, i16, i16, i16, i16, i16)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         //     SampleFormat::S32 => {
//         //         for channel in 0..num_channels as usize {
//         //             let data = audio_frame.data::<i32>(channel);
//         //             println!("data length: {:?}", data.len());
//         //         }
//         //     }
//         //     SampleFormat::S64 => {
//         //         for channel in 0..num_channels as usize {
//         //             let data = audio_frame.data::<i64>(channel);
//         //             println!("data length: {:?}", data.len());
//         //         }
//         //     }
//         //     SampleFormat::FLT => {
//         //         for channel in 0..num_channels as usize {
//         //             let data = audio_frame.data::<f32>(channel);
//         //             println!("data length: {:?}", data.len());
//         //         }
//         //     }
//         //     SampleFormat::DBL => {
//         //         for channel in 0..num_channels as usize {
//         //             let data = audio_frame.data::<f64>(channel);
//         //             println!("data length: {:?}", data.len());
//         //         }
//         //     }
//         //     SampleFormat::U8P => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<u8>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(u8, u8)>(0);
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(u8, u8, u8)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(u8, u8, u8, u8)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(u8, u8, u8, u8, u8)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(u8, u8, u8, u8, u8, u8)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(u8, u8, u8, u8, u8, u8, u8)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         //     SampleFormat::S16P => {
//         //         for channel in 0..num_channels as usize {
//         //             let data = audio_frame.data::<i16>(channel);
//         //             let samples: Vec<i16> = data.iter().take(10).cloned().collect();
//         //             println!("{:?}", samples);
//         //
//         //             println!("data length: {:?}", data.len());
//         //         }
//         //     }
//         //     SampleFormat::S32P => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<i32>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(i32, i32)>(0);
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(i32, i32, i32)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(i32, i32, i32, i32)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(i32, i32, i32, i32, i32)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(i32, i32, i32, i32, i32, i32)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(i32, i32, i32, i32, i32, i32, i32)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         //     SampleFormat::S64P => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<i64>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(i64, i64)>(0);
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(i64, i64, i64)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(i64, i64, i64, i64)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(i64, i64, i64, i64, i64)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(i64, i64, i64, i64, i64, i64)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(i64, i64, i64, i64, i64, i64, i64)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         //     SampleFormat::FLTP => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<f32>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(f32, f32)>(0);
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(f32, f32, f32)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(f32, f32, f32, f32)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(f32, f32, f32, f32, f32)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(f32, f32, f32, f32, f32, f32)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(f32, f32, f32, f32, f32, f32, f32)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         //     SampleFormat::DBLP => {
//         //         match num_channels {
//         //             1 => {
//         //                 let data = audio_frame.data::<f64>(0);
//         //             }
//         //             2 => {
//         //                 let data = audio_frame.data::<(f64, f64)>(0);
//         //             }
//         //             3 => {
//         //                 let data = audio_frame.data::<(f64, f64, f64)>(0);
//         //             }
//         //             4 => {
//         //                 let data = audio_frame.data::<(f64, f64, f64, f64)>(0);
//         //             }
//         //             5 => {
//         //                 let data = audio_frame.data::<(f64, f64, f64, f64, f64)>(0);
//         //             }
//         //             6 => {
//         //                 let data = audio_frame.data::<(f64, f64, f64, f64, f64, f64)>(0);
//         //             }
//         //             7 => {
//         //                 let data = audio_frame.data::<(f64, f64, f64, f64, f64, f64, f64)>(0);
//         //             }
//         //             _ => {}
//         //         }
//         //     }
//         // }
//
//         println!("samplefmt: {:?}", samplefmt.name());
//     }
//     Ok(())
// }
