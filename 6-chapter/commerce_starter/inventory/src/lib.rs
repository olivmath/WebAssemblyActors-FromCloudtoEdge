use commerce_interface as demo;
use wapc_guest as guest;

use serde::{Deserialize, Serialize};
use wasmcloud_actor_core as actor;

use guest::prelude::*;

#[actor::init]
fn init() {
    demo::Handlers::register_query_inventory(query_inventory);
}

fn query_inventory(sku: String) -> HandlerResult<demo::InventoryResponse> {
    todo!()
}
