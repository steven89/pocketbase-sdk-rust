use crate::{client::Client, api::{ApiError, RequestError}};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedRecordList<T> {
    pub page: u32,
    pub per_page: u32,
    pub total_items: u32,
    pub total_pages: u32,
    pub items: Vec<T>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ListResponse<T> {
    ErrorResponse(ApiError),
    SuccessResponse(PaginatedRecordList<T>)
}

pub async fn records<T: DeserializeOwned>(collection: impl Into<String>, client: &Client) -> Result<ListResponse<T>, RequestError> {
    let response = client.get(
        format!("collections/{}/records", collection.into()),
        None
    ).await?;

    let res = response.text().await?;
    match serde_json::from_str::<ListResponse<T>>(&res) {
        Ok(r) => Ok(r),
        Err(e) => Err(RequestError::ParseError(e, res)),
    }
    // Ok(response.json::<ListResponse<T>>().await?)
}
