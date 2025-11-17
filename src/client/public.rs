use crate::error::PixivError;
use crate::network::HttpClient;
use crate::models::public::{
    PublicSearchResponse, PublicUserDetail, PublicUserIllusts, PublicUserBookmarks,
    SearchTarget, Sort, Duration, Filter, Restrict
};
use reqwest;
use serde_json;
use tracing::debug;

/// Public API client for accessing Pixiv public API endpoints
pub struct PublicClient {
    http_client: HttpClient,
}

impl PublicClient {
    /// Create new PublicClient instance
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    /// Get user details
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `filter` - Filter (default is ForIOS)
    ///
    /// # Returns
    /// Returns PublicUserDetail containing user detailed information
    pub async fn user_detail(
        &self,
        user_id: u64,
        filter: Option<Filter>,
    ) -> Result<PublicUserDetail, PixivError> {
        let filter = filter.unwrap_or(Filter::ForIOS);

        debug!(user_id = user_id, filter = ?filter, "Fetching user detail");

        let params = vec![
            ("user_id", user_id.to_string()),
            ("filter", filter.to_string()),
        ];

        let url = format!("{}{}", self.http_client.base_url(), "/v1/user/detail");
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let user_detail: PublicUserDetail = serde_json::from_str(&text)?;
        
        Ok(user_detail)
    }

    /// Search illustrations
    ///
    /// # Arguments
    /// * `word` - Search keyword
    /// * `search_target` - Search target (default is PartialMatchForTags)
    /// * `sort` - Sort method (default is DateDesc)
    /// * `duration` - Search duration
    /// * `start_date` - Start date
    /// * `end_date` - End date
    /// * `filter` - Filter (default is ForIOS)
    /// * `offset` - Offset
    /// * `search_ai_type` - Search AI type (0: Filter AI-generated works, 1: Show AI-generated works)
    ///
    /// # Returns
    /// Returns PublicSearchResponse containing search results
    pub async fn search_illust(
        &self,
        word: &str,
        search_target: Option<SearchTarget>,
        sort: Option<Sort>,
        duration: Option<Duration>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        filter: Option<Filter>,
        offset: Option<u32>,
        search_ai_type: Option<u32>,
    ) -> Result<PublicSearchResponse, PixivError> {
        let search_target = search_target.unwrap_or(SearchTarget::PartialMatchForTags);
        let sort = sort.unwrap_or(Sort::DateDesc);
        let filter = filter.unwrap_or(Filter::ForIOS);

        debug!(word = %word, search_target = ?search_target, sort = ?sort, "Searching illustrations");

        let mut params = Vec::new();
        params.push(("word", word.to_string()));
        params.push(("search_target", search_target.to_string()));
        params.push(("sort", sort.to_string()));
        params.push(("filter", filter.to_string()));

        if let Some(duration) = duration {
            params.push(("duration", duration.to_string()));
        }
        if let Some(start_date) = start_date {
            params.push(("start_date", start_date.to_string()));
        }
        if let Some(end_date) = end_date {
            params.push(("end_date", end_date.to_string()));
        }
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        if let Some(search_ai_type) = search_ai_type {
            params.push(("search_ai_type", search_ai_type.to_string()));
        }

        let url = format!("{}{}", self.http_client.base_url(), "/v1/search/illust");
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let search_result: PublicSearchResponse = serde_json::from_str(&text)?;
        
        Ok(search_result)
    }


    /// Get user works list
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `offset` - Offset (default is 0)
    ///
    /// # Returns
    /// Returns PublicUserIllusts containing user works list
    pub async fn user_illusts(
        &self,
        user_id: u64,
        offset: Option<u32>,
    ) -> Result<PublicUserIllusts, PixivError> {
        let offset = offset.unwrap_or(0);

        debug!(user_id = user_id, offset = offset, "Fetching user illusts");

        let params = vec![
            ("user_id", user_id.to_string()),
            ("offset", offset.to_string()),
        ];

        let url = format!("{}{}", self.http_client.base_url(), "/v1/user/illusts");
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let user_illusts: PublicUserIllusts = serde_json::from_str(&text)?;
        
        Ok(user_illusts)
    }

    /// Get user bookmarked illustrations
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `restrict` - Restriction type (default is Public)
    /// * `offset` - Offset (default is 0)
    ///
    /// # Returns
    /// Returns PublicUserBookmarks containing user bookmarked illustrations list
    pub async fn user_bookmarks_illust(
        &self,
        user_id: u64,
        restrict: Option<Restrict>,
        offset: Option<u32>,
    ) -> Result<PublicUserBookmarks, PixivError> {
        let restrict = restrict.unwrap_or(Restrict::Public);
        let offset = offset.unwrap_or(0);

        debug!(user_id = user_id, restrict = ?restrict, offset = offset, "Fetching user bookmarks");

        let params = vec![
            ("user_id", user_id.to_string()),
            ("restrict", restrict.to_string()),
            ("offset", offset.to_string()),
        ];

        let url = format!("{}{}", self.http_client.base_url(), "/v1/user/bookmarks/illust");
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let user_bookmarks: PublicUserBookmarks = serde_json::from_str(&text)?;
        
        Ok(user_bookmarks)
    }
}
