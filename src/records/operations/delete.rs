use serde::{Serialize, Deserialize};

use crate::{client::Client, api::{ApiError, RequestError}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuccessResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum DeleteResponse {
    SuccessResponse(SuccessResponse),
    FailureResponse(ApiError)
}

pub async fn record(collection: &str, id: &str, client: &Client) -> Result<(), RequestError> {
    let url = format!("/api/collections/{}/records/{}", collection, id);
    let resp = client.delete(url, None).await?;
    match resp.json::<DeleteResponse>().await? {
        DeleteResponse::SuccessResponse(_) => Ok(()),
        DeleteResponse::FailureResponse(e) => Err(e.into()),
    }
}
