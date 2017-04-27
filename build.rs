// build.rs
extern crate time;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_timer.rs");
    let mut f = File::create(&dest_path).unwrap();
    let tm = time::at(time::get_time());
    let timer = format!("{}",tm.rfc3339()).to_string();
    f.write_all(b"pub fn get_build_time() -> &'static str {\n\t\"").unwrap();
    f.write_all(timer.as_bytes()).unwrap();
    f.write_all(b"\"\n}").unwrap();
}