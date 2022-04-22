use super::*;

pub fn get(player_id: &str) -> HandlerResult<http::Response> {
    let balance = kv::default().get(player_id.to_string())?;
    Ok(http::Response::json(
        &Transaction {
            gold: if balance.exists {
                balance.value.parse()?
            } else {
                0
            },
        },
        200,
        "OK",
    ))
}
