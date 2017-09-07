use std::borrow::Borrow;
use std::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str

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

pub fn get_values<K: Eq + Hash + Debug, V>(params: &HashMap<K, V>, keys: Vec<K>) {

    let mut values = Vec::new();
    let test = gen_hashmap!(
        "aa" => [1, 2],
        "bb" => [1 + 1, 33]
    );

//    let test = keys.join("-");

    for key in keys.iter() {

        let val = params.get(key).unwrap();

        values.push(val);
    }

    println!("{:?}", test);
    ()
}
