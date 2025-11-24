use crate::error::{PixivError, Result};
use crate::models::app::{
    CommentsResponse, ContentType, Duration, Filter, FollowRestrict, IllustBookmarkResponse,
    IllustDetail, IllustFollowResponse, IllustRelatedResponse, RankingMode, RankingResponse, RecommendedResponse,
    SearchIllustResponse, SearchTarget, Sort, TrendingTagsResponse, UgoiraMetadataResponse,
    UserFollowingResponse, UserFollowerResponse, UserIllustrationsResponse, UserMypixivResponse,
    UserNovelsResponse, NovelSeriesResponse, NovelNewResponse, NovelFollowResponse, NovelRecommendedResponse, SearchNovelResponse, UserBookmarksNovelResponse, WebviewNovelResponse, NovelSearchTarget, NovelFollowRestrict, Novel,
};
use crate::network::HttpClient;
use regex::Regex;
use std::collections::HashMap;
use tracing::debug;

/// App API client for interacting with Pixiv App API
#[derive(Debug, Clone)]
pub struct AppClient {
    /// HTTP client
    http_client: HttpClient,
    /// API base URL
    base_url: String,
}

impl AppClient {
    /// Create new App API client instance
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http_client,
            base_url: "https://app-api.pixiv.net".to_string(),
        }
    }

    /// Set API base URL
    pub fn set_base_url(&mut self, url: String) {
        self.base_url = url;
    }

    /// Get API base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get illustration details
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    ///
    /// # Returns
    /// Returns illustration detail information
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let detail = client.illust_detail(12345678).await?;
    /// ```
    pub async fn illust_detail(&self, illust_id: u64) -> Result<IllustDetail> {
        debug!(illust_id = %illust_id, "Fetching illustration detail");
        
        let url = format!("{}/v1/illust/detail", self.base_url);
        let params = [("illust_id", illust_id.to_string())];
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let detail: IllustDetail = serde_json::from_str(&text)?;
        
        Ok(detail)
    }

    /// Get illustration ranking
    ///
    /// # Arguments
    /// * `mode` - Ranking mode
    /// * `filter` - Filter
    /// * `date` - Date (format: YYYY-MM-DD)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns ranking response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let ranking = client.illust_ranking(
    ///     RankingMode::Day,
    ///     Filter::ForIOS,
    ///     Some("2023-01-01"),
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn illust_ranking(
        &self,
        mode: RankingMode,
        filter: Filter,
        date: Option<&str>,
        offset: Option<u32>,
    ) -> Result<RankingResponse> {
        debug!(
            mode = %mode.to_string(),
            filter = %filter.to_string(),
            date = ?date,
            offset = ?offset,
            "Fetching illustration ranking"
        );
        
        let url = format!("{}/v1/illust/ranking", self.base_url);
        let mut params = Vec::new();
        params.push(("mode", mode.to_string()));
        params.push(("filter", filter.to_string()));
        
        if let Some(date) = date {
            params.push(("date", date.to_string()));
        }
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let ranking: RankingResponse = serde_json::from_str(&text)?;
        
        Ok(ranking)
    }

    /// Get recommended illustrations
    ///
    /// # Arguments
    /// * `content_type` - Content type
    /// * `include_ranking_label` - Whether to include ranking label
    /// * `filter` - Filter
    /// * `max_bookmark_id_for_recommend` - Maximum bookmark ID for recommendation
    /// * `min_bookmark_id_for_recent_illust` - Minimum bookmark ID for recent illustrations
    /// * `offset` - Offset
    /// * `include_ranking_illusts` - Whether to include ranking illustrations
    /// * `bookmark_illust_ids` - List of bookmarked illustration IDs
    /// * `viewed` - List of viewed illustration IDs
    ///
    /// # Returns
    /// Returns recommendation response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let recommended = client.illust_recommended(
    ///     ContentType::Illust,
    ///     true,
    ///     Filter::ForIOS,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn illust_recommended(
        &self,
        content_type: ContentType,
        include_ranking_label: bool,
        filter: Filter,
        max_bookmark_id_for_recommend: Option<u64>,
        min_bookmark_id_for_recent_illust: Option<u64>,
        offset: Option<u32>,
        include_ranking_illusts: Option<bool>,
        bookmark_illust_ids: Option<Vec<u64>>,
        viewed: Option<Vec<String>>,
        include_privacy_policy: Option<String>,
    ) -> Result<RecommendedResponse> {
        debug!(
            content_type = %content_type.to_string(),
            include_ranking_label = %include_ranking_label,
            filter = %filter.to_string(),
            max_bookmark_id_for_recommend = ?max_bookmark_id_for_recommend,
            min_bookmark_id_for_recent_illust = ?min_bookmark_id_for_recent_illust,
            offset = ?offset,
            include_ranking_illusts = ?include_ranking_illusts,
            bookmark_illust_ids = ?bookmark_illust_ids,
            viewed = ?viewed,
            include_privacy_policy = ?include_privacy_policy,
            "Fetching recommended illustrations"
        );
        
        let url = format!("{}/v1/illust/recommended", self.base_url);
        let mut params = Vec::new();
        params.push(("content_type".to_string(), content_type.to_string()));
        params.push(("include_ranking_label".to_string(), include_ranking_label.to_string()));
        params.push(("filter".to_string(), filter.to_string()));
        
        if let Some(max_bookmark_id_for_recommend) = max_bookmark_id_for_recommend {
            params.push(("max_bookmark_id_for_recommend".to_string(), max_bookmark_id_for_recommend.to_string()));
        }
        
        if let Some(min_bookmark_id_for_recent_illust) = min_bookmark_id_for_recent_illust {
            params.push(("min_bookmark_id_for_recent_illust".to_string(), min_bookmark_id_for_recent_illust.to_string()));
        }
        
        if let Some(offset) = offset {
            params.push(("offset".to_string(), offset.to_string()));
        }
        
        if let Some(include_ranking_illusts) = include_ranking_illusts {
            params.push(("include_ranking_illusts".to_string(), include_ranking_illusts.to_string()));
        }
        
        if let Some(bookmark_illust_ids) = bookmark_illust_ids {
            let ids = bookmark_illust_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push(("bookmark_illust_ids".to_string(), ids));
        }
        
        if let Some(viewed) = viewed {
            for (i, viewed_id) in viewed.iter().enumerate() {
                let key = format!("viewed[{}]", i);
                params.push((key, viewed_id.to_string()));
            }
        }

        if let Some(include_privacy_policy) = include_privacy_policy {
            params.push(("include_privacy_policy".to_string(), include_privacy_policy));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let recommended: RecommendedResponse = serde_json::from_str(&text)?;
        
        Ok(recommended)
    }

    /// Search illustrations
    ///
    /// # Arguments
    /// * `word` - Search keyword
    /// * `search_target` - Search target
    /// * `sort` - Sort method
    /// * `duration` - Search duration
    /// * `start_date` - Start date (format: YYYY-MM-DD)
    /// * `end_date` - End date (format: YYYY-MM-DD)
    /// * `filter` - Filter
    /// * `search_ai_type` - AI type (0: Filter AI-generated works, 1: Show AI-generated works)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns search response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let search_result = client.search_illust(
    ///     "original",
    ///     SearchTarget::PartialMatchForTags,
    ///     Sort::DateDesc,
    ///     None,
    ///     None,
    ///     None,
    ///     Filter::ForIOS,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn search_illust(
        &self,
        word: &str,
        search_target: SearchTarget,
        sort: Sort,
        duration: Option<Duration>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        filter: Filter,
        search_ai_type: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SearchIllustResponse> {
        debug!(
            word = %word,
            search_target = %search_target.to_string(),
            sort = %sort.to_string(),
            duration = ?duration,
            start_date = ?start_date,
            end_date = ?end_date,
            filter = %filter.to_string(),
            search_ai_type = ?search_ai_type,
            offset = ?offset,
            "Searching illustrations"
        );
        
        let url = format!("{}/v1/search/illust", self.base_url);
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
        
        if let Some(search_ai_type) = search_ai_type {
            params.push(("search_ai_type", search_ai_type.to_string()));
        }
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let search_result: SearchIllustResponse = serde_json::from_str(&text)?;
        
        Ok(search_result)
    }

    /// Get illustrations from followed users
    ///
    /// # Arguments
    /// * `restrict` - Follow restriction (public/private)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns response with illustrations from followed users
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let follow_illusts = client.illust_follow(
    ///     FollowRestrict::Public,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn illust_follow(
        &self,
        restrict: FollowRestrict,
        offset: Option<u32>,
    ) -> Result<IllustFollowResponse> {
        debug!(
            restrict = %restrict.to_string(),
            offset = ?offset,
            "Fetching follow illustrations"
        );
        
        let url = format!("{}/v2/illust/follow", self.base_url);
        let mut params = Vec::new();
        params.push(("restrict", restrict.to_string()));
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let follow_response: IllustFollowResponse = serde_json::from_str(&text)?;
        
        Ok(follow_response)
    }

    /// Get illustration comments
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    /// * `offset` - Offset
    /// * `include_total_comments` - Whether to include total comment count
    ///
    /// # Returns
    /// Returns comment response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let comments = client.illust_comments(
    ///     12345678,
    ///     Some(0),
    ///     Some(true)
    /// ).await?;
    /// ```
    pub async fn illust_comments(
        &self,
        illust_id: u64,
        offset: Option<u32>,
        include_total_comments: Option<bool>,
    ) -> Result<CommentsResponse> {
        debug!(
            illust_id = %illust_id,
            offset = ?offset,
            include_total_comments = ?include_total_comments,
            "Fetching illustration comments"
        );
        
        let url = format!("{}/v1/illust/comments", self.base_url);
        let mut params = Vec::new();
        params.push(("illust_id", illust_id.to_string()));
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        if let Some(include_total_comments) = include_total_comments {
            params.push(("include_total_comments", include_total_comments.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let comments: CommentsResponse = serde_json::from_str(&text)?;
        
        Ok(comments)
    }

    /// Get related illustrations
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    /// * `filter` - Filter
    /// * `seed_illust_ids` - List of seed illustration IDs
    /// * `offset` - Offset
    /// * `viewed` - List of viewed illustration IDs
    ///
    /// # Returns
    /// Returns related illustrations response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let related = client.illust_related(
    ///     12345678,
    ///     Filter::ForIOS,
    ///     None,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn illust_related(
        &self,
        illust_id: u64,
        filter: Filter,
        seed_illust_ids: Option<Vec<u64>>,
        offset: Option<u32>,
        viewed: Option<Vec<String>>,
    ) -> Result<IllustRelatedResponse> {
        debug!(
            illust_id = %illust_id,
            filter = %filter.to_string(),
            seed_illust_ids = ?seed_illust_ids,
            offset = ?offset,
            viewed = ?viewed,
            "Fetching related illustrations"
        );

        let url = format!("{}/v2/illust/related", self.base_url);
        let mut params = Vec::new();
        params.push(("illust_id".to_string(), illust_id.to_string()));
        params.push(("filter".to_string(), filter.to_string()));

        if let Some(seed_illust_ids) = seed_illust_ids {
            let ids = seed_illust_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push(("seed_illust_ids".to_string(), ids));
        }

        if let Some(offset) = offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        if let Some(viewed) = viewed {
            for (i, viewed_id) in viewed.iter().enumerate() {
                let key = format!("viewed[{}]", i);
                params.push((key, viewed_id.to_string()));
            }
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let related: IllustRelatedResponse = serde_json::from_str(&text)?;

        Ok(related)
    }

    /// Get user following list
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `restrict` - Follow restriction (public/private)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns user following response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let following = client.user_following(
    ///     12345678,
    ///     FollowRestrict::Public,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn user_following(
        &self,
        user_id: u64,
        restrict: FollowRestrict,
        offset: Option<u32>,
    ) -> Result<UserFollowingResponse> {
        debug!(
            user_id = %user_id,
            restrict = %restrict.to_string(),
            offset = ?offset,
            "Fetching user following"
        );
        
        let url = format!("{}/v1/user/following", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        params.push(("restrict", restrict.to_string()));
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let following: UserFollowingResponse = serde_json::from_str(&text)?;
        
        Ok(following)
    }

    /// Get user followers list
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `filter` - Filter
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns user followers response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let followers = client.user_follower(
    ///     12345678,
    ///     Filter::ForIOS,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn user_follower(
        &self,
        user_id: u64,
        filter: Filter,
        offset: Option<u32>,
    ) -> Result<UserFollowerResponse> {
        debug!(
            user_id = %user_id,
            filter = %filter.to_string(),
            offset = ?offset,
            "Fetching user followers"
        );
        
        let url = format!("{}/v1/user/follower", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        params.push(("filter", filter.to_string()));
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let followers: UserFollowerResponse = serde_json::from_str(&text)?;
        
        Ok(followers)
    }

    /// Get user illustrations
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `content_type` - Content type (illust/manga)
    /// * `filter` - Filter
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns user illustrations response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let illusts = client.user_illusts(
    ///     12345678,
    ///     Some(ContentType::Illust),
    ///     Filter::ForIOS,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn user_illusts(
        &self,
        user_id: u64,
        content_type: Option<ContentType>,
        filter: Filter,
        offset: Option<u32>,
    ) -> Result<UserIllustrationsResponse> {
        debug!(
            user_id = %user_id,
            content_type = ?content_type,
            filter = %filter.to_string(),
            offset = ?offset,
            "Fetching user illustrations"
        );

        let url = format!("{}/v1/user/illusts", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        params.push(("filter", filter.to_string()));

        if let Some(content_type) = content_type {
            params.push(("type", content_type.to_string()));
        }

        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let illusts: UserIllustrationsResponse = serde_json::from_str(&text)?;

        Ok(illusts)
    }

    /// Get user mypixiv list
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns user mypixiv response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let mypixiv = client.user_mypixiv(
    ///     12345678,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn user_mypixiv(
        &self,
        user_id: u64,
        offset: Option<u32>,
    ) -> Result<UserMypixivResponse> {
        debug!(
            user_id = %user_id,
            offset = ?offset,
            "Fetching user mypixiv"
        );
        
        let url = format!("{}/v1/user/mypixiv", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        
        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let mypixiv: UserMypixivResponse = serde_json::from_str(&text)?;
        
        Ok(mypixiv)
    }

    /// Add illustration bookmark
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    /// * `restrict` - Bookmark restriction (public/private)
    /// * `tags` - Tag list
    ///
    /// # Returns
    /// Returns bookmark response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let result = client.illust_bookmark_add(
    ///     12345678,
    ///     FollowRestrict::Public,
    ///     Some(vec!["tag1".to_string(), "tag2".to_string()])
    /// ).await?;
    /// ```
    pub async fn illust_bookmark_add(
        &self,
        illust_id: u64,
        restrict: FollowRestrict,
        tags: Option<Vec<String>>,
    ) -> Result<IllustBookmarkResponse> {
        debug!(
            illust_id = %illust_id,
            restrict = %restrict.to_string(),
            tags = ?tags,
            "Adding illustration bookmark"
        );
        
        let url = format!("{}/v2/illust/bookmark/add", self.base_url);
        let mut data = HashMap::new();
        data.insert("illust_id", illust_id.to_string());
        data.insert("restrict", restrict.to_string());
        
        if let Some(tags) = tags {
            data.insert("tags", tags.join(" "));
        }
        
        let response = self
            .http_client
            .send_request(reqwest::Method::POST, &url, Some(&data))
            .await?;
        
        let text = response.text().await?;
        let bookmark_response: IllustBookmarkResponse = serde_json::from_str(&text)?;
        
        Ok(bookmark_response)
    }

    /// Delete illustration bookmark
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    ///
    /// # Returns
    /// Returns bookmark response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let result = client.illust_bookmark_delete(12345678).await?;
    /// ```
    pub async fn illust_bookmark_delete(
        &self,
        illust_id: u64,
    ) -> Result<IllustBookmarkResponse> {
        debug!(
            illust_id = %illust_id,
            "Deleting illustration bookmark"
        );
        
        let url = format!("{}/v1/illust/bookmark/delete", self.base_url);
        let mut data = HashMap::new();
        data.insert("illust_id", illust_id.to_string());
        
        let response = self
            .http_client
            .send_request(reqwest::Method::POST, &url, Some(&data))
            .await?;
        
        let text = response.text().await?;
        let bookmark_response: IllustBookmarkResponse = serde_json::from_str(&text)?;
        
        Ok(bookmark_response)
    }

    /// Get trending tags
    ///
    /// # Arguments
    /// * `filter` - Filter
    ///
    /// # Returns
    /// Returns trending tags response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let trending = client.trending_tags_illust(Filter::ForIOS).await?;
    /// ```
    pub async fn trending_tags_illust(
        &self,
        filter: Filter,
    ) -> Result<TrendingTagsResponse> {
        debug!(
            filter = %filter.to_string(),
            "Fetching trending tags"
        );
        
        let url = format!("{}/v1/trending-tags/illust", self.base_url);
        let params = [("filter", filter.to_string())];
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let trending: TrendingTagsResponse = serde_json::from_str(&text)?;
        
        Ok(trending)
    }

    /// Get Ugoira metadata
    ///
    /// # Arguments
    /// * `illust_id` - Illustration ID
    ///
    /// # Returns
    /// Returns Ugoira metadata response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let metadata = client.ugoira_metadata(12345678).await?;
    /// ```
    pub async fn ugoira_metadata(
        &self,
        illust_id: u64,
    ) -> Result<UgoiraMetadataResponse> {
        debug!(
            illust_id = %illust_id,
            "Fetching ugoira metadata"
        );
        
        let url = format!("{}/v1/ugoira/metadata", self.base_url);
        let params = [("illust_id", illust_id.to_string())];
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let metadata: UgoiraMetadataResponse = serde_json::from_str(&text)?;
        
        Ok(metadata)
    }

    /// Get user novels
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `filter` - Filter
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns user novels response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let novels = client.user_novels(
    ///     12345678,
    ///     Filter::ForIOS,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn user_novels(
        &self,
        user_id: u64,
        filter: Filter,
        offset: Option<u32>,
    ) -> Result<UserNovelsResponse> {
        debug!(
            user_id = %user_id,
            filter = %filter.to_string(),
            offset = ?offset,
            "Fetching user novels"
        );

        let url = format!("{}/v1/user/novels", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        params.push(("filter", filter.to_string()));

        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let novels: UserNovelsResponse = serde_json::from_str(&text)?;

        Ok(novels)
    }

    /// Get novel series
    ///
    /// # Arguments
    /// * `series_id` - Series ID
    /// * `filter` - Filter
    /// * `last_order` - Last order
    ///
    /// # Returns
    /// Returns novel series response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let series = client.novel_series(
    ///     12345678,
    ///     Filter::ForIOS,
    ///     Some("1")
    /// ).await?;
    /// ```
    pub async fn novel_series(
        &self,
        series_id: u64,
        filter: Filter,
        last_order: Option<&str>,
    ) -> Result<NovelSeriesResponse> {
        debug!(
            series_id = %series_id,
            filter = %filter.to_string(),
            last_order = ?last_order,
            "Fetching novel series"
        );

        let url = format!("{}/v2/novel/series", self.base_url);
        let mut params = Vec::new();
        params.push(("series_id", series_id.to_string()));
        params.push(("filter", filter.to_string()));

        if let Some(last_order) = last_order {
            params.push(("last_order", last_order.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let series: NovelSeriesResponse = serde_json::from_str(&text)?;

        Ok(series)
    }

    /// Get novel detail
    ///
    /// # Arguments
    /// * `novel_id` - Novel ID
    ///
    /// # Returns
    /// Returns novel detail information
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let detail = client.novel_detail(12345678).await?;
    /// ```
    pub async fn novel_detail(
        &self,
        novel_id: u64,
    ) -> Result<Novel> {
        debug!(novel_id = %novel_id, "Fetching novel detail");

        let url = format!("{}/v2/novel/detail", self.base_url);
        let params = [("novel_id", novel_id.to_string())];

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let json_response: serde_json::Value = serde_json::from_str(&text)?;
        let novel: Novel = serde_json::from_value(json_response["novel"].clone())?;

        Ok(novel)
    }

    /// Get new novels
    ///
    /// # Arguments
    /// * `filter` - Filter
    /// * `max_novel_id` - Maximum novel ID
    ///
    /// # Returns
    /// Returns new novels response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let new_novels = client.novel_new(
    ///     Filter::ForIOS,
    ///     None
    /// ).await?;
    /// ```
    pub async fn novel_new(
        &self,
        filter: Filter,
        max_novel_id: Option<u64>,
    ) -> Result<NovelNewResponse> {
        debug!(
            filter = %filter.to_string(),
            max_novel_id = ?max_novel_id,
            "Fetching new novels"
        );

        let url = format!("{}/v1/novel/new", self.base_url);
        let mut params = Vec::new();
        params.push(("filter", filter.to_string()));

        if let Some(max_novel_id) = max_novel_id {
            params.push(("max_novel_id", max_novel_id.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let new_novels: NovelNewResponse = serde_json::from_str(&text)?;

        Ok(new_novels)
    }

    /// Get novels from followed users
    ///
    /// # Arguments
    /// * `restrict` - Follow restriction (public/private/all)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns response with novels from followed users
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let follow_novels = client.novel_follow(
    ///     NovelFollowRestrict::Public,
    ///     Some(0)
    /// ).await?;
    /// ```
    pub async fn novel_follow(
        &self,
        restrict: NovelFollowRestrict,
        offset: Option<u32>,
    ) -> Result<NovelFollowResponse> {
        debug!(
            restrict = %restrict.to_string(),
            offset = ?offset,
            "Fetching follow novels"
        );

        let url = format!("{}/v1/novel/follow", self.base_url);
        let mut params = Vec::new();
        params.push(("restrict", restrict.to_string()));

        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let follow_response: NovelFollowResponse = serde_json::from_str(&text)?;

        Ok(follow_response)
    }

    /// Get novel comments
    ///
    /// # Arguments
    /// * `novel_id` - Novel ID
    /// * `offset` - Offset
    /// * `include_total_comments` - Whether to include total comment count
    ///
    /// # Returns
    /// Returns comment response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let comments = client.novel_comments(
    ///     12345678,
    ///     Some(0),
    ///     Some(true)
    /// ).await?;
    /// ```
    pub async fn novel_comments(
        &self,
        novel_id: u64,
        offset: Option<u32>,
        include_total_comments: Option<bool>,
    ) -> Result<CommentsResponse> {
        debug!(
            novel_id = %novel_id,
            offset = ?offset,
            include_total_comments = ?include_total_comments,
            "Fetching novel comments"
        );

        let url = format!("{}/v1/novel/comments", self.base_url);
        let mut params = Vec::new();
        params.push(("novel_id", novel_id.to_string()));

        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }

        if let Some(include_total_comments) = include_total_comments {
            params.push(("include_total_comments", include_total_comments.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let comments: CommentsResponse = serde_json::from_str(&text)?;

        Ok(comments)
    }

    /// Get recommended novels
    ///
    /// # Arguments
    /// * `include_ranking_label` - Whether to include ranking label
    /// * `filter` - Filter
    /// * `offset` - Offset
    /// * `include_ranking_novels` - Whether to include ranking novels
    /// * `already_recommended` - List of already recommended novel IDs
    /// * `max_bookmark_id_for_recommend` - Maximum bookmark ID for recommendation
    /// * `include_privacy_policy` - Whether to include privacy policy
    ///
    /// # Returns
    /// Returns recommended novels response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let recommended_novels = client.novel_recommended(
    ///     true,
    ///     Filter::ForIOS,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn novel_recommended(
        &self,
        include_ranking_label: bool,
        filter: Filter,
        offset: Option<u32>,
        include_ranking_novels: Option<bool>,
        already_recommended: Option<Vec<u64>>,
        max_bookmark_id_for_recommend: Option<u64>,
        include_privacy_policy: Option<String>,
    ) -> Result<NovelRecommendedResponse> {
        debug!(
            include_ranking_label = %include_ranking_label,
            filter = %filter.to_string(),
            offset = ?offset,
            include_ranking_novels = ?include_ranking_novels,
            already_recommended = ?already_recommended,
            max_bookmark_id_for_recommend = ?max_bookmark_id_for_recommend,
            include_privacy_policy = ?include_privacy_policy,
            "Fetching recommended novels"
        );

        let url = format!("{}/v1/novel/recommended", self.base_url);
        let mut params = Vec::new();
        params.push(("include_ranking_label".to_string(), include_ranking_label.to_string()));
        params.push(("filter".to_string(), filter.to_string()));

        if let Some(offset) = offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        if let Some(include_ranking_novels) = include_ranking_novels {
            params.push(("include_ranking_novels".to_string(), include_ranking_novels.to_string()));
        }

        if let Some(already_recommended) = already_recommended {
            let ids = already_recommended
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push(("already_recommended".to_string(), ids));
        }

        if let Some(max_bookmark_id_for_recommend) = max_bookmark_id_for_recommend {
            params.push(("max_bookmark_id_for_recommend".to_string(), max_bookmark_id_for_recommend.to_string()));
        }

        if let Some(include_privacy_policy) = include_privacy_policy {
            params.push(("include_privacy_policy".to_string(), include_privacy_policy));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let recommended_novels: NovelRecommendedResponse = serde_json::from_str(&text)?;

        Ok(recommended_novels)
    }

    /// Search novels
    ///
    /// # Arguments
    /// * `word` - Search keyword
    /// * `search_target` - Search target
    /// * `sort` - Sort method
    /// * `merge_plain_keyword_results` - Whether to merge plain keyword results
    /// * `include_translated_tag_results` - Whether to include translated tag results
    /// * `start_date` - Start date (format: YYYY-MM-DD)
    /// * `end_date` - End date (format: YYYY-MM-DD)
    /// * `filter` - Filter
    /// * `search_ai_type` - AI type (0: Filter AI-generated works, 1: Show AI-generated works)
    /// * `offset` - Offset
    ///
    /// # Returns
    /// Returns search novel response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let search_result = client.search_novel(
    ///     "original",
    ///     NovelSearchTarget::PartialMatchForTags,
    ///     Sort::DateDesc,
    ///     true,
    ///     true,
    ///     None,
    ///     None,
    ///     Filter::ForIOS,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn search_novel(
        &self,
        word: &str,
        search_target: NovelSearchTarget,
        sort: Sort,
        merge_plain_keyword_results: bool,
        include_translated_tag_results: bool,
        start_date: Option<&str>,
        end_date: Option<&str>,
        filter: Filter,
        search_ai_type: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SearchNovelResponse> {
        debug!(
            word = %word,
            search_target = %search_target.to_string(),
            sort = %sort.to_string(),
            merge_plain_keyword_results = %merge_plain_keyword_results,
            include_translated_tag_results = %include_translated_tag_results,
            start_date = ?start_date,
            end_date = ?end_date,
            filter = %filter.to_string(),
            search_ai_type = ?search_ai_type,
            offset = ?offset,
            "Searching novels"
        );

        let url = format!("{}/v1/search/novel", self.base_url);
        let mut params = Vec::new();
        params.push(("word", word.to_string()));
        params.push(("search_target", search_target.to_string()));
        params.push(("sort", sort.to_string()));
        params.push(("merge_plain_keyword_results", merge_plain_keyword_results.to_string()));
        params.push(("include_translated_tag_results", include_translated_tag_results.to_string()));
        params.push(("filter", filter.to_string()));

        if let Some(start_date) = start_date {
            params.push(("start_date", start_date.to_string()));
        }

        if let Some(end_date) = end_date {
            params.push(("end_date", end_date.to_string()));
        }

        if let Some(search_ai_type) = search_ai_type {
            params.push(("search_ai_type", search_ai_type.to_string()));
        }

        if let Some(offset) = offset {
            params.push(("offset", offset.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let search_result: SearchNovelResponse = serde_json::from_str(&text)?;

        Ok(search_result)
    }

    /// Get user bookmarks novel
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `restrict` - Bookmark restriction (public/private)
    /// * `max_bookmark_id` - Maximum bookmark ID
    /// * `tag` - Tag
    ///
    /// # Returns
    /// Returns user bookmarks novel response
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let bookmarks = client.user_bookmarks_novel(
    ///     12345678,
    ///     FollowRestrict::Public,
    ///     None,
    ///     None
    /// ).await?;
    /// ```
    pub async fn user_bookmarks_novel(
        &self,
        user_id: u64,
        restrict: FollowRestrict,
        max_bookmark_id: Option<u64>,
        tag: Option<&str>,
    ) -> Result<UserBookmarksNovelResponse> {
        debug!(
            user_id = %user_id,
            restrict = %restrict.to_string(),
            max_bookmark_id = ?max_bookmark_id,
            tag = ?tag,
            "Fetching user bookmarks novel"
        );

        let url = format!("{}/v1/user/bookmarks/novel", self.base_url);
        let mut params = Vec::new();
        params.push(("user_id", user_id.to_string()));
        params.push(("restrict", restrict.to_string()));

        if let Some(max_bookmark_id) = max_bookmark_id {
            params.push(("max_bookmark_id", max_bookmark_id.to_string()));
        }

        if let Some(tag) = tag {
            params.push(("tag", tag.to_string()));
        }

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;
        let bookmarks: UserBookmarksNovelResponse = serde_json::from_str(&text)?;

        Ok(bookmarks)
    }

    /// Get webview novel
    ///
    /// # Arguments
    /// * `novel_id` - Novel ID
    /// * `raw` - Whether to return raw HTML content
    ///
    /// # Returns
    /// Returns webview novel response or raw HTML content
    ///
    /// # Example
    /// ```rust
    /// let client = AppClient::new(http_client);
    /// let webview_novel = client.webview_novel(
    ///     12345678,
    ///     false
    /// ).await?;
    /// ```
    pub async fn webview_novel(
        &self,
        novel_id: u64,
        raw: bool,
    ) -> Result<WebviewNovelResponse> {
        debug!(
            novel_id = %novel_id,
            raw = %raw,
            "Fetching webview novel"
        );

        let url = format!("{}/webview/v2/novel", self.base_url);
        let mut params = Vec::new();
        params.push(("id", novel_id.to_string()));
        params.push(("viewer_version", "20221031_ai".to_string()));

        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;

        let text = response.text().await?;

        if raw {
            // If raw is true, return the HTML content directly.
            // This requires a different return type, so we'll need to adjust the function signature
            // or create a new method for raw HTML. For now, we'll assume the non-raw case.
            // TODO: Handle raw HTML return type if needed.
            return Err(PixivError::Other("Raw HTML not supported yet".to_string()));
        }

        // Extract JSON content from HTML
        let re = regex::Regex::new(r"novel:\s*(\{.+?\}),\s*isOwnWork")?;
        let captures = re.captures(&text).ok_or_else(|| {
            PixivError::Other("Failed to extract novel JSON from webview HTML".to_string())
        })?;

        let json_str = captures.get(1).map_or("", |m| m.as_str());
        let mut webview_novel: WebviewNovelResponse = serde_json::from_str(json_str)?;
        webview_novel.novel_text = webview_novel.novel.caption.clone(); // Assuming novel_text is derived from caption

        Ok(webview_novel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_target_to_string() {
        assert_eq!(SearchTarget::PartialMatchForTags.to_string(), "partial_match_for_tags");
        assert_eq!(SearchTarget::ExactMatchForTags.to_string(), "exact_match_for_tags");
        assert_eq!(SearchTarget::TitleAndCaption.to_string(), "title_and_caption");
        assert_eq!(SearchTarget::Keyword.to_string(), "keyword");
    }

    #[test]
    fn test_sort_to_string() {
        assert_eq!(Sort::DateDesc.to_string(), "date_desc");
        assert_eq!(Sort::DateAsc.to_string(), "date_asc");
        assert_eq!(Sort::PopularDesc.to_string(), "popular_desc");
    }

    #[test]
    fn test_ranking_mode_to_string() {
        assert_eq!(RankingMode::Day.to_string(), "day");
        assert_eq!(RankingMode::Week.to_string(), "week");
        assert_eq!(RankingMode::Month.to_string(), "month");
        assert_eq!(RankingMode::DayMale.to_string(), "day_male");
        assert_eq!(RankingMode::DayFemale.to_string(), "day_female");
        assert_eq!(RankingMode::WeekOriginal.to_string(), "week_original");
        assert_eq!(RankingMode::WeekRookie.to_string(), "week_rookie");
        assert_eq!(RankingMode::DayManga.to_string(), "day_manga");
        assert_eq!(RankingMode::DayR18.to_string(), "day_r18");
        assert_eq!(RankingMode::DayMaleR18.to_string(), "day_male_r18");
        assert_eq!(RankingMode::DayFemaleR18.to_string(), "day_female_r18");
        assert_eq!(RankingMode::WeekR18.to_string(), "week_r18");
        assert_eq!(RankingMode::WeekR18g.to_string(), "week_r18g");
    }

    #[test]
    fn test_content_type_to_string() {
        assert_eq!(ContentType::Illust.to_string(), "illust");
        assert_eq!(ContentType::Manga.to_string(), "manga");
    }

    #[test]
    fn test_filter_to_string() {
        assert_eq!(Filter::ForIOS.to_string(), "for_ios");
        assert_eq!(Filter::None.to_string(), "");
    }

    #[test]
    fn test_duration_to_string() {
        assert_eq!(Duration::WithinLastDay.to_string(), "within_last_day");
        assert_eq!(Duration::WithinLastWeek.to_string(), "within_last_week");
        assert_eq!(Duration::WithinLastMonth.to_string(), "within_last_month");
    }
}