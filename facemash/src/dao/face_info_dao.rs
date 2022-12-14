use futures_util::StreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::{doc, Document};
use mongodb::results::{InsertOneResult, UpdateResult};

use crate::entity::face_info::FaceInfo;
use crate::resource::mongo;
use crate::resource::mongo::MONGO_CLIENT;

pub async fn add_one_face_info(face_info: &FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    let collection: Collection<FaceInfo> = mongo::MONGO_CLIENT.get().await
        .database(FaceInfo::db_name()).collection(FaceInfo::coll_name());
    collection.insert_one(face_info, None).await
}

pub async fn get_one_face_info_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FaceInfo>> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());
    collection.find_one(doc_filter, None).await
}


pub async fn get_face_infos_by_doc_filter(
    doc_filter: Document,
) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());

    let mut ret_face_infos: Vec<FaceInfo> = Vec::new();
    let mut results = collection.find(doc_filter, None).await?;
    // result实现了CursorStream，继承了ChangeStream，继承了Stream。但是都没有next()方法，所以必须要引入futures_util::StreamExt为所有实现了Stream的结构体实现next()方法(在源码第245行)
    while let Some(result) = results.next().await {
        // 这里因为serde已经帮助实现了反序列化，所以可以反序列化成结构体
        let face_info: FaceInfo = bson::from_document(result?)?;
        ret_face_infos.push(face_info);
    }
    Ok(ret_face_infos)
}

pub async fn update_face_info_by_doc_filter(
    doc_filter: Document,
    update_info: Document,
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<FaceInfo> = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());

    collection.update_one(doc_filter, update_info, None).await
}

pub async fn get_face_info_sample(size: i64) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    let collection: Collection<FaceInfo> = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());

    let pipeline = vec![doc! {"$sample": {"size": size}}];

    let mut ret_face_infos: Vec<FaceInfo> = Vec::new();
    let mut results = collection.aggregate(pipeline, None).await?;
    while let Some(result) = results.next().await {
        // Use serde to deserialize into the FaceInfo struct:
        let face_info: FaceInfo = bson::from_document(result?)?;
        ret_face_infos.push(face_info);
    }
    Ok(ret_face_infos)
}