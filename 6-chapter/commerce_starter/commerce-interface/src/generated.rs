extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct Handlers {}

impl Handlers {
    pub fn register_add_cart_item(f: fn(String, u32) -> HandlerResult<InventoryResponse>) {
        *ADD_CART_ITEM.write().unwrap() = Some(f);
        register_function(&"AddCartItem", add_cart_item_wrapper);
    }
    pub fn register_query_inventory(f: fn(String) -> HandlerResult<InventoryResponse>) {
        *QUERY_INVENTORY.write().unwrap() = Some(f);
        register_function(&"QueryInventory", query_inventory_wrapper);
    }
}

lazy_static! {
    static ref ADD_CART_ITEM: RwLock<Option<fn(String, u32) -> HandlerResult<InventoryResponse>>> =
        RwLock::new(None);
    static ref QUERY_INVENTORY: RwLock<Option<fn(String) -> HandlerResult<InventoryResponse>>> =
        RwLock::new(None);
}

fn add_cart_item_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<AddCartItemArgs>(input_payload)?;
    let lock = ADD_CART_ITEM.read().unwrap().unwrap();
    let result = lock(input.sku, input.quantity)?;
    Ok(serialize(result)?)
}

fn query_inventory_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<QueryInventoryArgs>(input_payload)?;
    let lock = QUERY_INVENTORY.read().unwrap().unwrap();
    let result = lock(input.sku)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct AddCartItemArgs {
    #[serde(rename = "sku")]
    pub sku: String,
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryInventoryArgs {
    #[serde(rename = "sku")]
    pub sku: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct InventoryResponse {
    #[serde(rename = "sku")]
    pub sku: String,
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
