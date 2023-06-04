// Structs/Helpers for the github releases api
// Improve errors. We currently just return a String if there is an error
// to avoid the complications of potentially returning multiple different
// errors from the same function. This is a 'dumb' way to do error handling
// and we should do it the 'right' way

pub mod api {
    use serde::{Deserialize};
    use reqwest;

    const RELEASE_PATH: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";
    const LATEST_PATH: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";
    
    #[derive(Debug, Deserialize)]
    pub struct User {
        login: String,
        id: usize,
        node_id: String,
        avatar_url: String,
        gravatar_id: String,
        url: String,
        html_url: String,
        followers_url: String,
        following_url: String,
        gists_url: String,
        starred_url: String,
        subscriptions_url: String,
        organizations_url: String,
        repos_url: String,
        events_url: String,
        received_events_url: String,
        #[serde(alias = "type")]
        user_type: String,
        site_admin: bool,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct Assets {
        url: String,
        id: usize,
        node_id: String,
        name: String,
        label: Option<String>,
        uploader: User,
        content_type: String,
        state: String,
        size: usize,
        download_count: usize,
        created_at: String,
        updated_at: String,
        browser_download_url: String,

    }
    
    #[derive(Debug, Deserialize)]
    pub struct Reactions {
        url: String,
        total_count: usize,
        #[serde(alias = "+1")]
        plus_one: usize,
        #[serde(alias = "-1")]
        minus_one: usize,
        laugh: usize,
        hooray: usize,
        confused: usize,
        heart: usize,
        rocket: usize,
        eyes: usize,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct Release {
        url: String,
        assets_url: String,
        upload_url: String,
        html_url: String,
        id: usize,
        author: User,
        node_id: String,
        tag_name: String,
        target_commitish: String,
        name: String,
        draft: bool,
        prerelease: bool,
        created_at: String,
        published_at: String,
        assets: Vec<Assets>,
        tarball_url: String,
        zipball_url: String,
        body: String,
        reactions: Reactions
    }

    impl Release {
        pub fn get_version(self: &Self) -> String {
            self.tag_name.clone()
        }

        pub fn get_download_url(self: &Self) -> String {
            self.tarball_url.clone()
        }

        pub fn get_body(self: &Self) -> String {
            self.body.clone()
        }
    }

    pub type Releases = Vec<Release>;

    pub async fn releases(per_page: Option<u8>, page: Option<usize>) -> Result<Releases, String> {
        let pp: String = if let Some(number) = per_page {
            number.to_string()
        } else { String::from("10") };
        let p: String = if let Some(number) = page {
            number.to_string()
        } else { String::from("1") };
        let response = reqwest::Client::new()
            .get(RELEASE_PATH)
            .query(&[("per_page", pp.to_string()),
            ("page", p.to_string())])
            .header("user-agent", "protonctl-rs")
            .send()
            .await;
        let json_res = match response {
            Ok(e) => e.json::<Releases>().await,
            Err(err) => {
                return Err(format!("Failed to open page: {} : {}", p, err));
            }
        };
        match json_res {
            Ok(e) => Ok(e),
            Err(err) => Err(format!("Failed to deserialize json: {}", err))
        }
    }

    pub async fn latest_release() -> Result<Release, String> {
        let response = reqwest::Client::new()
            .get(LATEST_PATH)
            .header("user-agent", "protonctl-rs")
            .send()
            .await;
        let json_res = match response {
            Ok(e) => e.json::<Release>().await,
            Err(err) => {
                return Err(format!("Failed to get results from {} : {}", LATEST_PATH, err));

            }
        };
        match json_res {
            Ok(e) => Ok(e),
            Err(err) => Err(format!("Failed to deserialize json: {}", err))
        }
    }
}

    
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_releases() -> Result<(), String> {
        use crate::github::api::releases;
        let result = releases(Some(50), Some(1)).await?;
        assert_eq!(result.len(), 50);
        Ok(())
    }
    
    #[tokio::test]
    async fn can_get_latest_release() -> Result<(), String> {
        use crate::github::api::latest_release;
        let result = latest_release().await?;
        Ok(())
    }
}
