use std::path::PathBuf;
use std::io::BufReader;
use rodio::Decoder;

pub fn play_the_list(list: Vec<String>) {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    list.iter().for_each(|file_path| {
        let file = std::fs::File::open(file_path).unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(decoder);
    });

    sink.sleep_until_end();
}