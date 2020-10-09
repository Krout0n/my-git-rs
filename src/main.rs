use std::env;
use std::fs;

const DOT_GIT: &'static str = "./workdir/.git";

fn git_init() -> Option<()> {
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
    fs::remove_dir_all(DOT_GIT).expect("failed to initialize");
    let args: Vec<_> = env::args().skip(1).collect();
    let sub = args.get(0).unwrap();
    if sub == &"init".to_string() {
        dbg!(git_init());
    } else {
        unimplemented!();
    };
}
