use libflate::zlib::Decoder;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

const DOT_GIT: &'static str = "./workdir/.git";

fn git_init() -> Option<()> {
    // for debug
    fs::remove_dir_all(DOT_GIT).expect("failed to initialize");

    let path = env::current_dir().unwrap();

    let create_dirs = vec![
        DOT_GIT.to_string(),
        format!("{}/objects", DOT_GIT),
        format!("{}/objects/info", DOT_GIT),
        format!("{}/objects/pack", DOT_GIT),
    ];

    for dir in create_dirs {
        fs::create_dir(&dir).expect(&format!("failed to create {}", dir));
    }
    println!(
        "Initialized empty Git repository in {}/workdir/.git",
        path.display()
    );
    Some(())
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let sub = args.get(0).unwrap();
    if sub == &"init".to_string() {
        dbg!(git_init());
    } else if sub == &"cat-file".to_string() {
        // echo 'test content' | git hash-object -w --stdin
        // blob 13\x00test content\n
        // d670460b4b4aece5915caf5c68d12f560a9fe3e4
        // git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
        // test content
        // blob 13\u{0}test content\n
        // let hash = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";

        let hash = args.get(1).unwrap();

        let dir = &hash[0..2];
        let file = &hash[2..];
        let path_str = format!("{}/objects/{}/{}", DOT_GIT, dir, file);
        let path = Path::new(&path_str);
        let file = File::open(path).expect(&format!("cannoot find the path: {}", path.display()));
        let f = BufReader::new(file);
        let mut decoder = Decoder::new(f).unwrap();
        let mut buf = String::new();
        decoder
            .read_to_string(&mut buf)
            .expect("i don't know what to do.");
        dbg!(buf);
    } else if sub == &"hash-object".to_string() {
    } else {
        unimplemented!();
    };
}
