use commerce_interface as demo;
use wapc_guest as guest;

use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;

use guest::prelude::*;

#[actor::init]
fn init() {
    demo::Handlers::register_add_cart_item(add_cart_item);
}

fn add_cart_item(sku: String, qty: u32) -> HandlerResult<demo::InventoryResponse> {
    todo!()
}
