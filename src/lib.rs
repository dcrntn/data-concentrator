use mongodb::bson::Document;
use mongodb::{bson::doc, bson::DateTime, Client};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValData {
    node_val: String,
    node_last_update: DateTime,
    node_name: String,
    node_rw_direction: String,
    node_uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateData {
    node_val: String,
    node_name: String,
    node_rw_direction: String,
    node_uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeVal {
    node_val: String,
    node_uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MbTcpData {
    mb_lock_to_uid: String,
    mb_ip: String,
    mb_port: String,
    mb_register: String,
    mb_rw: String,
}

// Generates a random 20 char long string used as a name or uid.
fn rand_name() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(20)
        .collect();
    rand_string
}

async fn update_mdb(client: &Client, filter: Document, update: Document) -> u64 {
    let db = client.database("dconc");
    let collection = db.collection::<NodeVal>("bucket");
    // Insert some books into the "mydb.books" collection.
    let result = collection.update_one(filter, update, None).await.unwrap();

    result.modified_count
}

pub async fn create_new_datanode(client: &Client) -> String {
    let db = client.database("dconc");
    let collection = db.collection::<ValData>("bucket");
    let node_uid = rand_name();
    let ret_uid = node_uid.clone();

    let docs = vec![ValData {
        node_val: "".to_string(),
        node_last_update: DateTime::now(),
        node_rw_direction: "".to_string(),
        node_name: "".to_string(),
        node_uid: node_uid,
    }];

    // Insert some books into the "mydb.books" collection.
    collection.insert_many(docs, None).await.unwrap();

    ret_uid
}

pub async fn update_value_node(client: &Client, node_data: Json<NodeVal>) -> u64 {
    let filter = doc! { "node_uid" : &node_data.node_uid};

    let update = doc! { "$set" : {
        "node_val" : &node_data.node_val,
        "node_last_update": DateTime::now()
    }
    };

    update_mdb(&client, filter, update).await
}

pub async fn update_data_node(client: &Client, node_data: Json<UpdateData>) -> u64 {
    let filter = doc! { "node_uid" : &node_data.node_uid};

    let update = doc! { "$set" : {
        "node_val" : &node_data.node_val,
        "node_last_update": DateTime::now(),
        "node_name": &node_data.node_name,
        "node_rw_direction": &node_data.node_rw_direction,
    }
    };

    update_mdb(&client, filter, update).await
}

pub async fn read_node(client: &Client, uuid: &str) -> Option<NodeVal> {
    let db = client.database("dconc");
    let collection = db.collection::<NodeVal>("bucket");

    let filter = doc! { "node_uid" : &uuid};

    let node_data = collection.find_one(Some(filter), None).await.unwrap();

    node_data
}

pub async fn create_modbus_tcp(client: &Client, mb_tcp_data: Json<MbTcpData>) -> u64 {
    let db = client.database("dconc");
    let collection = db.collection::<MbTcpData>("mbstuff");
    let docs = vec![MbTcpData {
        mb_lock_to_uid: format!("{}", mb_tcp_data.mb_lock_to_uid),
        mb_ip: format!("{}", mb_tcp_data.mb_ip),
        mb_port: format!("{}", mb_tcp_data.mb_port),
        mb_register: format!("{}", mb_tcp_data.mb_register),
        mb_rw: format!("{}", mb_tcp_data.mb_rw),
    }];

    collection.insert_many(docs, None).await.unwrap();
    1
}
