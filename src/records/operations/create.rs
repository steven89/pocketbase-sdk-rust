use crate::{client::Client, api::{RequestError, ApiError}};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CreateResponse<T> {
    SuccessResponse(T),
    FailureResponse(ApiError)
}

pub async fn record<T: Serialize + DeserializeOwned>(collection: &str, changeset: &T, client: &Client) -> Result<CreateResponse<T>, RequestError> {
    let url = format!("collections/{}/records", collection);
    let response = client.post::<T>(url, &changeset).await?;
    Ok(response.json::<CreateResponse<T>>().await?)
}
