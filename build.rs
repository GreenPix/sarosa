use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::ops::Deref;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sarosa_version.rs");
    let mut f = File::create(&dest_path).unwrap();

    let git_output = String::from_utf8(Command::new("git")
     .arg("log")
     .arg("-1")
     .arg("--pretty=oneline")
     .arg("--no-color")
     .output()
     .unwrap_or_else(|e| { panic!("failed to execute git commit lookup: {}", e) })
     .stdout)
     .expect("Invalid utf8 ! Use Ubuntu please, cmon.");

    let mut log_git_iter = git_output
     .split_whitespace();

    let hash_git = log_git_iter
     .next()
     .expect("NO GIT LOG ARE YOU KIDDING ME BRO ??!!");

    let sarosa_version = format!("{}.{}.{}-{}",
        env!( "CARGO_PKG_VERSION_MAJOR" ),
        env!( "CARGO_PKG_VERSION_MINOR" ),
        env!( "CARGO_PKG_VERSION_PATCH" ),
        hash_git);

    f.write_all(format!("
        pub fn sarosa_version() -> &'static str {{
            \"Sarosa client {}\"
        }}
    ", sarosa_version).into_bytes().deref()).unwrap();
}
