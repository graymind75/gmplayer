use std::process::exit;

mod files_handler;
mod player;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        println!("please pass the folder contain musics.")
    }else {
        let path = &args[0];
        let path_buf = match files_handler::get_directory(&path) {
            Ok(path_buf) => path_buf,
            Err(e) => {
                println!("error: {}", e);
                exit(1)
            }
        };
        let audio_files = files_handler::get_directory_musics(&path_buf);
        player::play_the_list(audio_files);
    }
}
