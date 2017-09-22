use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use iron::prelude::*;
use multipart::server::{Multipart, Entries, SaveResult, SavedFile};

use common::http::*;

pub fn render_home(req: &mut Request) -> IronResult<Response> {
    let mut data = ViewData::new(req);

    data.insert("title", json!("runner"));
    data.insert("message", json!("欢迎你，这里是首页"));

    respond_view("home/index", &data)
}

pub fn upload(req: &mut Request) -> IronResult<Response> {

    match Multipart::from_request(req) {

        Ok(mut multipart) => {

            match multipart.save().temp() {

                SaveResult::Full(entries) => process_entries(entries),

                SaveResult::Partial(entries, reason) => {

                    response_text("保存部分成功")
                }

                SaveResult::Error(err) => {

                    response_text("保存失败")
                }
            }
        }

        _ => {

            response_text("上传出错")
        }
    }
}

fn process_entries(entries: Entries) -> IronResult<Response> {

    for (name, field) in entries.fields {

        println!("Field {:?}: {:?}", name, field);
    }

    for (name, files) in entries.files {
        println!("Field {:?} has {} files:", name, files.len());

        for file in files {

            create_file(&file);
        }
    }

    response_text("保存成功")
}

fn create_file(saved_file: &SavedFile) {

    let dest_path = "upload/".to_owned() + &*saved_file.filename.clone().unwrap();
    let path = Path::new(&dest_path);
    let dest_name = path.display();
    let mut data = Vec::new();

    let mut temp_file = match File::open(&saved_file.path) {
        Ok(file) => file,
        Err(err) =>  panic!("can't open file: {}", err.description())
    };

    temp_file.read_to_end(&mut data).expect("unable to read data");

    let mut new_file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => panic!("can't create file {}: {}", dest_name, err.description())
    };

    match new_file.write_all(&data) {
        Ok(_) => (),
        Err(err) => panic!("can't wrote to file {}: {}", dest_path, err.description())
    }
}
