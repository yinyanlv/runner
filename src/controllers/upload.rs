use std::fs::{self, File, DirBuilder};
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use iron::prelude::*;
use serde_json::Value;
use multipart::server::{Multipart, Entries, SaveResult, SavedFile};

use common::http::*;
use common::lazy_static::{UPLOAD_TEMP_PATH, UPLOAD_ASSETS_PATH};

pub fn create_upload_folder() {

    DirBuilder::new()
        .recursive(true)
        .create(&*UPLOAD_TEMP_PATH).unwrap();

    DirBuilder::new()
        .recursive(true)
        .create(&*UPLOAD_ASSETS_PATH).unwrap();
}

pub fn upload_file(req: &mut Request) -> IronResult<Response> {

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

    let mut temp_file_list = vec![];

    for (name, field) in entries.fields {

        println!("Field {:?}: {:?}", name, field);
    }

    for (name, files) in entries.files {
        println!("Field {:?} has {} files:", name, files.len());

        for file in files {

            create_temp_file(&file, &mut temp_file_list);
        }
    }

    let mut data = JsonData::new();

    data.data = json!(&temp_file_list);

    respond_json(&data)
}

fn create_temp_file(saved_file: &SavedFile, temp_file_list: &mut Vec<Value> ) {

    let dest_path = UPLOAD_TEMP_PATH.to_owned() + "/" + &*saved_file.filename.clone().unwrap();
    let path = Path::new(&dest_path);
    let dest_name = path.display();
    let mut data = Vec::new();

    let mut temp_file = match File::open(&saved_file.path) {
        Ok(file) => file,
        Err(err) =>  panic!("can't open file: {}", err.description())
    };

    temp_file_list.push(json!({
        "filename": saved_file.filename.clone().unwrap(),
        "path": &path.to_owned()
    }));

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
