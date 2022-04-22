use super::*;

pub fn from(player_id: &str, raw: &[u8]) -> HandlerResult<http::Response> {
    let tx: Transaction = serde_json::from_slice(raw)?;
    kv::default().add(player_id.to_string(), tx.gold as i32 * -1)?;
    Ok(http::Response::ok())
}
