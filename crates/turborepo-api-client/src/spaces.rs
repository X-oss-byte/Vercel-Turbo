use chrono::{DateTime, Local};
use serde::Serialize;
use turbopath::AbsoluteSystemPathBuf;

use crate::{retry, APIClient, Error};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacesRunPayload {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    status: RunStatus,
    #[serde(rename = "type")]
    ty: &'static str, // Hardcoded to "TURBO"
    exit_code: u32,
    command: String,
    repository_path: AbsoluteSystemPathBuf,
    #[serde(rename = "context")]
    run_context: String,
    client: SpacesClientSummary,
    git_branch: String,
    git_sha: String,
    #[serde(rename = "originationUser")]
    user: String,
}

#[derive(Serialize)]
struct SpacesClientSummary {
    id: String,
    name: String,
    version: String,
}

pub struct SpacesRun {
    id: String,
    url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum RunStatus {
    Running,
    Completed,
}

impl APIClient {
    pub async fn create_spaces_run(
        &self,
        space_id: &str,
        payload: SpacesRunPayload,
    ) -> Result<SpacesRun, Error> {
        let url = self.make_url(&format!("/v0/spaces/{}/runs", space_id));
        let request_builder = self.client.post(&url).json(&payload);

        let response = retry::make_retryable_request(request_builder)
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
