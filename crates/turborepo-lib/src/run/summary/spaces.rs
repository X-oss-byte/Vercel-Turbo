use turborepo_api_client::{
    spaces::{SpacesRun, SpacesRunPayload},
    APIAuth, APIClient,
};

use crate::run::summary::Error;

pub struct SpacesClient<'a> {
    space_id: &'a str,
    api_client: &'a APIClient,
    api_auth: APIAuth,
}

enum SpacesRequest {
    Start { payload: SpacesRunPayload },
}

impl<'a> SpacesClient<'a> {
    pub fn new(
        space_id: Option<&'a str>,
        api_client: &'a APIClient,
        api_auth: Option<APIAuth>,
    ) -> Option<Self> {
        // If space_id is empty, we don't build a client
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
        Ok(self
            .api_client
            .create_spaces_run(&self.space_id, payload)
            .await?)
    }
}
