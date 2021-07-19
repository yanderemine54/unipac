use clap::{App, load_yaml};
use std::str;
use ftp::FtpStream;

fn main() {
    let structure = load_yaml!("cli.yml");
    let matches = App::from_yaml(structure).get_matches();
    if let Some(_matches) = matches.subcommand_matches("install") {
        let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
        let _ = ftp_stream.login("anonymous", "john.smith@example.com").unwrap();
        println!("Current directory: {}", ftp_stream.pwd().unwrap());
        let remote_file = ftp_stream.simple_retr("hello.txt").unwrap();
        println!("Read file with contents\n{}\n", str::from_utf8(&remote_file.into_inner()).unwrap());
        let _ = ftp_stream.quit();
    }
}
