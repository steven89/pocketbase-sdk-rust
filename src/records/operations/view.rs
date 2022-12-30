use crate::{
    api::{ApiError, RequestError},
    client::Client,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ViewResponse<T> {
    ErrorResponse(ApiError),
    SuccessResponse(T),
}

pub async fn record<T: DeserializeOwned>(
    collection: impl Into<String>,
    id: impl Into<String>,
    client: &Client,
) -> Result<ViewResponse<T>, RequestError> {
    let response = client
        .get(format!("collections/{}/records/{}", collection.into(), id.into()), None)
        .await?;

    Ok(response.json::<ViewResponse<T>>().await?)
}
