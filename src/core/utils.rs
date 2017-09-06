use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::{Hash, BuildHasher};
use std::collections::HashMap;

use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use urlencoded::{UrlEncodedBody};


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

pub fn get_values<K: Eq + Hash, V>(params: &HashMap<K, V>, keys: Vec<&str>) {

//    let mut values = Vec::new();

//    println!("{:?}", params.get("username"));
//    for key in keys.iter() {
//
//        println!(1);
//        values.push("1");
//    }

    for key in params.keys() {

        println!("{:?}", key);
    }

    ()
}

