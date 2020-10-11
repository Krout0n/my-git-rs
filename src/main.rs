use crypto::digest::Digest;
use crypto::sha1::Sha1;
use libflate::zlib::{Decoder, Encoder};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

use clap::{App, Arg};

const DOT_GIT: &'static str = "./.git";

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

fn cat_file(hash: &str) {
    // echo 'test content' | git hash-object -w --stdin
    // blob 13\x00test content\n
    // d670460b4b4aece5915caf5c68d12f560a9fe3e4
    // git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
    // test content
    // blob 13\u{0}test content\n
    // let hash = "d670460b4b4aece5915caf5c68d12f560a9fe3e4";

    // let hash = args.get(1).unwrap();

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
}

fn hash_object(content: &str, should_write: bool) {
    // ❯ echo 'test content' | git hash-object --stdin
    // d670460b4b4aece5915caf5c68d12f560a9fe3e4
    // blob 13\u{0}test content\n

    let length = content.as_bytes().len();
    let will_be_blobed = format!("blob {}{}{}", length, '\x00', content);
    dbg!(&will_be_blobed);

    let mut hasher = Sha1::new();
    hasher.input_str(&will_be_blobed);
    let hex = hasher.result_str();
    println!("{}", &hex);

    // Encoding
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    let mut will_be_blobed = will_be_blobed.as_bytes();
    std::io::copy(&mut will_be_blobed, &mut encoder).unwrap();

    let res = encoder.finish();
    let mut encoded = res.unwrap().0.clone();

    if should_write {
        let dir = &hex[0..2];
        let file = &hex[2..];
        let create_dir = format!("{}/objects/{}", DOT_GIT, dir);
        let _ = fs::create_dir(&create_dir);
        let create_file = format!("{}/{}", create_dir, file);
        let mut file = File::create(create_file).expect("what?");
        dbg!(file.write_all(&mut encoded));
    };
}

fn main() {
    let matches = App::new("Krouton's own git")
        .version("0.1")
        .author("Krout0n <me@tokinia.me>")
        .subcommand(App::new("init").about("perhaps it works as you expect"))
        .subcommand(
            App::new("cat-file")
                .about("pretty-print object's content")
                .arg(Arg::new("hash")),
        )
        .subcommand(
            App::new("hash-object")
                .about("Compute object ID and optionally creates a blob from a file")
                .arg(Arg::new("filename"))
                .arg(
                    Arg::new("write")
                        .short('w')
                        .long("write")
                        .about("Create file"),
                )
                .arg(
                    Arg::new("stdin")
                        .long("stdin")
                        .about(" Read the object from standard input instead of from a file."),
                ),
        )
        .get_matches();
    let subcmds = matches.subcommand().unwrap();
    match subcmds.0 {
        "init" => {
            dbg!(git_init());
        }
        "cat-file" => {
            let args = subcmds.1;
            if let Some(hash) = args.value_of("hash") {
                cat_file(hash);
            } else {
                todo!();
            }
        }
        "hash-object" => {
            let args = subcmds.1;
            let should_write = args.is_present("write");
            let content = args.value_of("filename").map(|filename| {
                let file = File::open(filename).expect("can't open the specified file.");
                let mut bufreader = BufReader::new(file);
                let mut content = String::new();
                bufreader.read_to_string(&mut content);
                content
            });
            let content = if content.is_some() {
                content.unwrap()
            } else {
                if args.is_present("stdin") {
                    let mut content = String::new();
                    let stdin = std::io::stdin();
                    let mut handler = stdin.lock();
                    handler.read_to_string(&mut content);
                    content
                } else {
                    unimplemented!()
                }
            };
            hash_object(&content, should_write);
        }

        _ => unimplemented!(),
    }
}
