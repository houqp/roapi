use std::collections::HashMap;
use std::sync::Arc;

use crate::api::HandlerContext;
use crate::error::ApiErrResp;

use rocket::get;
use rocket::State;

pub async fn raw_get(ctx: &HandlerContext) -> Vec<u8> {
    serde_json::to_vec(ctx.cq.schema_map()).unwrap()
}

#[get("/api/schema")]
pub async fn schema(ctx: &State<HandlerContext>) -> Vec<u8> {
    raw_get(ctx).await
}

// #[derive(Deserialize)]
// pub struct SchemaTablePath {
//     table_name: String,
// }
//
// pub async fn get_by_table_name(
//     data: web::Data<HandlerContext>,
//     path: web::Path<SchemaTablePath>,
//     _req: HttpRequest,
//     _query: web::Query<HashMap<String, String>>,
// ) -> Result<HttpResponse, ApiErrResp> {
//     Ok(HttpResponse::Ok().content_type("application/json").body(
//         serde_json::to_vec(
//             data.cq
//                 .schema_map()
//                 .get(&path.table_name)
//                 .ok_or_else(|| ApiErrResp::not_found("invalid table name"))?,
//         )
//         .map_err(ApiErrResp::json_serialization)?,
//     ))
// }
