extern crate scan_dir;
extern crate clap;

use clap::{App, Arg};
use scan_dir::ScanDir;
use std::process::Command;
use std::path::PathBuf;


fn main() {
    let m_app = App::new("cleanWalk")
        .about("clean rust target ...")
        .arg(Arg::with_name("INPUT").help("Target DIR").required(true).index(1))
        .get_matches();
    let ps = m_app.value_of("INPUT").unwrap();
    let p = PathBuf::from(ps);
    clean(p);
}


fn clean(p: PathBuf) {

    ScanDir::dirs()
        .walk(p, |mut iter| while let Some((entry, name)) = iter.next() {
            match name.as_str() {
                "target" => {
                    let pt = entry.path().join("../");
                    let ps = pt.to_str().unwrap();
                    Command::new("cargo")
                        .current_dir(ps)
                        .arg("clean")
                        .output()
                        .expect("error for clean.");
                    iter.exit_current_dir();
                }
                "src" | ".git" => iter.exit_current_dir(),
                _ => {
                    println!("dir {:?}", entry.path());
                }
            }
        })
        .unwrap();
}
