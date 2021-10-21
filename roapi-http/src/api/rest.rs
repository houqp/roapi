use std::collections::HashMap;
use std::sync::Arc;

use rocket::get;
use rocket::State;

use crate::api::{encode_record_batches, encode_type_from_req, HandlerContext};
use crate::error::ApiErrResp;

#[inline]
pub async fn raw_get_table(
    ctx: &HandlerContext,
    table_name: &str,
    query: &HashMap<String, String>,
) -> Vec<u8> {
    let encode_type = encode_type_from_req().unwrap();

    let batches = ctx.cq.query_rest_table(table_name, &query).await.unwrap();

    encode_record_batches(encode_type, &batches).unwrap()
}

#[get("/api/tables/<table_name>?<params..>")]
pub async fn get_table(
    ctx: &State<HandlerContext>,
    table_name: &str,
    params: HashMap<String, String>,
) -> Vec<u8> {
    raw_get_table(ctx, table_name, &params).await
}
