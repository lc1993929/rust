use std::fs;
use std::fs::File;
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{error, Error, web};
use futures_util::TryStreamExt as _;
use log::{error, info};
use mongodb::bson::Document;
use mongodb::results::InsertOneResult;

use crate::dao::file_resource_dao;
use crate::entity::file_resource::FileResource;

const SAVE_DIR: &str = "./tmp";

pub async fn init_local_directory() {
    fs::create_dir_all(SAVE_DIR).unwrap()
}

pub async fn create_file_resource_with_stream(
    mut payload: Multipart,
    file_prefix_id: &str,
) -> Result<String, Error> {
    let mut filename = "".to_string();
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        filename = match content_disposition.get_filename() {
            None => {
                return Err(error::ErrorInternalServerError("找不到文件名"));
            }
            Some(f_name) => {
                info!("上传文件：{}",f_name);
                f_name.replace(" ", "_").to_string()
            }
        };


        let filepath = get_local_filepath(file_prefix_id, &filename);
        // 阻塞等待创建好文件
        let mut f = web::block(|| File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }


    Ok(filename)
}


pub fn get_local_filepath(face_info_id: &str, filename: &str) -> String {
    format!("{SAVE_DIR}/{face_info_id}-{filename}")
}

pub async fn create_file_resource(
    file_resource: &FileResource,
) -> mongodb::error::Result<InsertOneResult> {
    file_resource_dao::add_one_file_resource(file_resource).await
}

pub async fn get_one_file_resource_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FileResource>> {
    file_resource_dao::get_one_file_resource_by_doc_filter(doc_filter).await
}

pub async fn delete_file(filepath: &str) {
    match fs::remove_file(filepath) {
        Ok(_) => {}
        Err(err) => {
            error!("Failed to remove file: {:?}, error: {:?}", filepath, err)
        }
    };
}