use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use chrono::Local;

pub fn gen_salt() -> String {

    thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect::<String>()
}

pub fn gen_md5(str: &str) -> String {

    let mut sh = Md5::new();

    sh.input_str(str);
    sh.result_str().to_string()
}

pub fn gen_datetime() -> String {

    Local::now().naive_local().to_string()
}