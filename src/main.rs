use mongodb::Client;
use rocket::serde::json::Json;
use rocket::State;

#[macro_use]
extern crate rocket;

// Create Client
struct MngClient {
    mngc: Client,
}

#[post("/w", format = "json", data = "<node_data>")]
async fn write(
    mng_client: &State<MngClient>,
    node_data: Json<data_concentrator::NodeVal>,
) -> Json<String> {
    let change_count = data_concentrator::update_value_node(&mng_client.mngc, node_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[post("/u", format = "json", data = "<node_data>")]
async fn update(
    mng_client: &State<MngClient>,
    node_data: Json<data_concentrator::UpdateData>,
) -> Json<String> {
    let change_count = data_concentrator::update_data_node(&mng_client.mngc, node_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[get("/r/<uuid>")]
async fn read(mng_client: &State<MngClient>, uuid: &str) -> Json<data_concentrator::NodeVal> {
    let read_node_data = data_concentrator::read_node(&mng_client.mngc, uuid)
        .await
        .expect("Couldn't read uuid");
    Json(read_node_data)
}

#[get("/c")]
async fn create(mng_client: &State<MngClient>) -> Json<String> {
    let new_name = data_concentrator::create_new_datanode(&mng_client.mngc).await;
    Json(format!("{{'uid': '{}'}}", new_name))
}

#[post("/cmbtcp", format = "json", data = "<mb_tcp_data>")]
async fn create_modbus_tcp(
    mng_client: &State<MngClient>,
    mb_tcp_data: Json<data_concentrator::MbTcpData>,
) -> Json<String> {
    let change_count = data_concentrator::create_modbus_tcp(&mng_client.mngc, mb_tcp_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[post("/cmqtt", format = "json", data = "<mqtt_data>")]
async fn crate_mqtt(
    mng_client: &State<MngClient>,
    mqtt_data: Json<data_concentrator::MqttData>,
) -> Json<String> {
    let change_count = data_concentrator::create_mqtt(&mng_client.mngc, mqtt_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[launch]
async fn rocket() -> _ {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    rocket::build().manage(MngClient { mngc: client }).mount(
        "/",
        routes![write, read, create, update, create_modbus_tcp, crate_mqtt],
    )
}
