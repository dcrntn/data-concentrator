use mongodb::bson::Document;
use mongodb::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
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

#[options("/u")]
async fn update_option() -> status::Accepted<String> {
    status::Accepted(Some("ok".to_string()))
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
async fn create(mng_client: &State<MngClient>) -> Json<data_concentrator::NewUidGet> {
    let new_name = data_concentrator::create_new_datanode(&mng_client.mngc).await;
    let ret_struct = data_concentrator::NewUidGet { uid: new_name };
    Json(ret_struct)
}

#[options("/cmbtcp")]
async fn create_modbus_tcp_option() -> status::Accepted<String> {
    status::Accepted(Some("ok".to_string()))
}

#[post("/cmbtcp", format = "json", data = "<mb_tcp_data>")]
async fn create_modbus_tcp(
    mng_client: &State<MngClient>,
    mb_tcp_data: Json<data_concentrator::MbTcpData>,
) -> Json<String> {
    let change_count = data_concentrator::create_modbus_tcp(&mng_client.mngc, mb_tcp_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[options("/cmqtt")]
async fn create_mqtt_option() -> status::Accepted<String> {
    status::Accepted(Some("ok".to_string()))
}

#[post("/cmqtt", format = "json", data = "<mqtt_data>")]
async fn create_mqtt(
    mng_client: &State<MngClient>,
    mqtt_data: Json<data_concentrator::MqttData>,
) -> Json<String> {
    let change_count = data_concentrator::create_mqtt(&mng_client.mngc, mqtt_data).await;
    Json(format!("{{'changed_count': '{}'}}", change_count))
}

#[get("/getall/<typ>")]
async fn get_all(mng_client: &State<MngClient>, typ: String) -> Json<Vec<Document>> {
    let read_data = data_concentrator::read_all(&mng_client.mngc, typ).await;
    Json(read_data)
}

#[launch]
async fn rocket() -> _ {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    rocket::build()
        .attach(CORS)
        .manage(MngClient { mngc: client })
        .mount(
            "/",
            routes![
                write,
                read,
                create,
                update,
                update_option,
                create_modbus_tcp,
                create_modbus_tcp_option,
                create_mqtt,
                create_mqtt_option,
                get_all
            ],
        )
}
