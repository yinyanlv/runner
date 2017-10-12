use std::fs::{File, DirBuilder, read_dir, metadata, remove_file};
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};
use std::thread::{self, sleep};

use iron::prelude::*;
use uuid::Uuid;
use serde_json::Value;
use multipart::server::{Multipart, Entries, SaveResult, SavedFile};
use schedule::{Agenda, Job};
use regex::{Regex, Captures};

use common::http::*;
use common::utils::get_file_ext;
use common::lazy_static::{CONFIG_TABLE, UPLOAD_PATH, UPLOAD_TEMP_PATH, UPLOAD_ASSETS_PATH};

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

                SaveResult::Partial(_entries, _reason) => {

                    response_text("部分保存成功")
                }

                SaveResult::Error(_err) => {

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

    for (_name, files) in entries.files {

        for file in files {

            create_temp_file(&file, &mut temp_file_list);
        }
    }

    let mut data = JsonData::new();

    data.data = json!(&temp_file_list);

    respond_json(&data)
}

fn create_temp_file(saved_file: &SavedFile, temp_file_list: &mut Vec<Value> ) {

    let original_filename = &*saved_file.filename.clone().unwrap();
    let ext = get_file_ext(original_filename).unwrap_or("");
    let uuid_filename =  Uuid::new_v4().to_string() + "." + ext;
    let dest_path = UPLOAD_TEMP_PATH.to_owned() + "/" + &*uuid_filename;
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

pub fn run_clean_temp_task() {

    let upload_config = CONFIG_TABLE.get("upload").unwrap().as_table().unwrap();
    let ttl = upload_config.get("clean_temp_dir_ttl").unwrap().as_integer().unwrap() as u64;
    let upload_temp_path = upload_config.get("temp_path").unwrap().as_str().unwrap();

    thread::Builder::new()
        .name("run_clean_temp_task".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {

            let mut agenda = Agenda::new();
            let temp_dir_path = Path::new(&*upload_temp_path);

            agenda.add(Job::new(move || {

                let now = SystemTime::now();
                let one_day = Duration::from_millis(1000 * 60 * 60 * 24);

                for file_wrapper in read_dir(&temp_dir_path).unwrap() {
                    let file = file_wrapper.unwrap();
                    let file_path = file.path();
                    let create_time = metadata(&file_path).unwrap().created().unwrap();

                    if now.duration_since(create_time).unwrap() > one_day {  // 已创建但未保存时间超过一天

                        remove_file(&file_path).unwrap();
                    }
                }

            }, "* * * * * *".parse().unwrap()));

            loop {
                agenda.run_pending();

                sleep(Duration::from_millis(ttl));
            }
        }).unwrap();
}

/// 将临时文件夹中的相关文件，剪切到UPLOAD_ASSETS_PATH文件夹中
pub fn sync_upload_file(content: &str) -> String {

    let upload_temp_path = UPLOAD_PATH.to_owned() + "/" + &*UPLOAD_TEMP_PATH.to_owned() + "/";
    let upload_assets_path =  UPLOAD_PATH.to_owned() + "/" + &*UPLOAD_ASSETS_PATH.to_owned() + "/";
    let reg_str = format!("\\({0}([-._0-9a-zA-Z]+).?\\)", upload_temp_path);

    let reg = Regex::new(&*reg_str).unwrap();
    let mut files: Vec<String> = Vec::new();
    let new_content = reg.replace_all(&content, |caps: &Captures| {

        let filename = caps.get(1).unwrap().as_str();

        files.push(filename.to_owned());

        format!("({0}{1})", upload_assets_path, filename)
    });


    for filename in files {

        let source_str = UPLOAD_TEMP_PATH.to_owned() + "/" + &*filename;
        let dest_str = UPLOAD_ASSETS_PATH.to_owned() + "/" + &*filename;

        {
            let source_path = Path::new(&*source_str);
            let dest_path =  Path::new(&*dest_str);

            copy_and_delete_file(&*source_path, &*dest_path);
        }
    }

    new_content.to_string()
}

fn copy_and_delete_file(source_path: &Path, dest_path: &Path) {

    let mut data = Vec::new();

    let mut temp_file = match File::open(source_path) {
        Ok(file) => file,
        Err(err) =>  panic!("can't open file: {}", err.description())
    };

    temp_file.read_to_end(&mut data).expect("unable to read data");

    let mut new_file = match File::create(dest_path) {
        Ok(file) => file,
        Err(err) => panic!("can't create file {}", err.description())
    };

    match new_file.write_all(&data) {
        Ok(_) => {
            remove_file(source_path).unwrap();
            ()
        },
        Err(err) => panic!("can't wrote to file {:?}: {}", dest_path, err.description())
    }
}
