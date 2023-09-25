use turborepo_api_client::{spaces::SpacesRunPayload, APIAuth, APIClient};

struct SpacesClient {
    space_id: String,
    api_client: APIClient,
    api_auth: APIAuth,
}

impl SpacesClient {
    pub fn new(
        space_id: Option<String>,
        api_client: APIClient,
        api_auth: Option<APIAuth>,
    ) -> Option<Self> {
        let space_id = space_id?;
        let Some(api_auth) = api_auth else {
            eprintln!(
                "Error: experimentalSpaceId is enabled, but repo is not linked to API. Run `turbo \
                 link` or `turbo login` first"
            );
            return None;
        };

        Some(Self {
            space_id,
            api_client,
            api_auth,
        })
    }

    async fn create_run(&self, payload: SpacesRunPayload) -> Result<SpacesRun, Error> {
        let handle = tokio::spawn(self.api_client.create_spaces_run(&self.space_id, payload));

        Ok(handle)
    }
}
