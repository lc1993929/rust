use log::info;
use mongodb::bson::doc;

pub mod id_generate;
pub mod mongo;

pub async fn check_resources() {
    check_mongo().await;
    check_id_generator().await;
}

async fn check_id_generator() {
    let id = id_generate::get_id().await;
    info!("雪花算法初始化成功.id:{}", id);
}

async fn check_mongo() {
    mongo::MONGO_CLIENT
        .get()
        .await
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
        .unwrap();
    info!("mongo数据库连接成功");
}
