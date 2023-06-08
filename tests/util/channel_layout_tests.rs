use ffmpeg_di::util::channel_layout::ChannelLayout;

#[test]
pub fn test_channel_layout_default() {
    let c = ChannelLayout::default(2);
    println!("{:?}", c.nb_channels())
}