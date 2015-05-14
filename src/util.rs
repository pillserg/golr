use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;
use time;


pub fn read_file_to_string(file_path: &str) -> Result<String> {
    let path = Path::new(&file_path);
    let mut buff = String::new();
    File::open(&path).and_then(| mut f:File | {f.read_to_string(&mut buff).and(Ok(buff))})
}

pub fn time_ms() -> u64 {time::precise_time_ns()/1000000}
