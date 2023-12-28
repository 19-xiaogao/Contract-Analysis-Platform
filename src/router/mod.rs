use actix_multipart_extract::{File, Multipart, MultipartForm};
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use ethers::{abi::Abi, prelude::*};

use serde::Deserialize;

#[derive(Deserialize, MultipartForm, Debug)]
pub struct UploadForm {
    // #[multipart(max_size = 5MB)]
    file_field: File,
    contract_address: Address,
}

#[post("/upload")]
async fn upload(upload_form: Multipart<UploadForm>) -> impl Responder {
    // File field
    let file_contents = String::from_utf8(upload_form.file_field.bytes.to_vec())
        .expect("File contains non-UTF-8 characters");

    // println!("File contents: {}", file_contents);

    let contract_abi = Abi::load(file_contents.as_bytes()).expect("as to bytes");

    // 根据json 解析出 这个合约 有什么函数, 有什么事件, 有什么变量, 哪些函数改变了哪些触发了哪些事件,修改了哪些变量。
    println!(" all events {:?}", contract_abi.events.iter());
    println!(" all function {:?}", contract_abi.functions.iter());
    let event_list = contract_abi.events.values();
    for event in event_list {
        for _event in event.iter() {
            println!("event name:{:?}\n",_event.name);
            for input in _event.inputs.iter() {
                println!("event input:{:?}\n", input);

            }
        }

    }
    // println!("contract abi {:?}", contract_abi);
    println!("File content type: {}", upload_form.file_field.content_type);
    println!("File name: {}", upload_form.file_field.name);

    // List of strings field
    println!("List of strings: {:?}", upload_form.contract_address);

    HttpResponse::Ok()
}

pub async fn web_server() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(upload))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
