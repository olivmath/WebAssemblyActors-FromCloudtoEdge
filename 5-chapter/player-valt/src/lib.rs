mod balance;
mod deposit;
mod withdraw;

use serde::{Deserialize, Serialize};
use serde_json;
use wapc_guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    pub gold: u32,
}
fn handle_http_request(req: http::Request) -> HandlerResult<http::Response> {
    let tokens: Vec<&str> = req.path.split("/").collect();
    if !req.path.starts_with("/vault") {
        return Ok(http::Response::bad_request());
    }
    let player_id = tokens[tokens.len() - 1];

    match req.method.as_ref() {
        "GET" => balance::get(player_id),
        "PUT" => deposit::to(player_id, &req.body),
        "DELETE" => withdraw::from(player_id, &req.body),
        _ => Ok(http::Response::bad_request()),
    }
}

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(handle_http_request);
}
