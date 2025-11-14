use crate::error::{PixivError, Result};
use crate::models::app::{
    Comment, CommentsResponse, ContentType, Duration, Filter, FollowRestrict, IllustBookmarkResponse,
    IllustDetail, IllustFollowResponse, RankingMode, RankingResponse, RecommendedResponse,
    SearchIllustResponse, SearchTarget, Sort, TrendingTagsResponse, UgoiraMetadataResponse,
    UserFollowingResponse, UserFollowerResponse, UserMypixivResponse,
};
use crate::network::HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// App API客户端，用于与Pixiv App API交互
#[derive(Debug, Clone)]
pub struct AppClient {
    /// HTTP客户端
    http_client: HttpClient,
    /// API基础URL
    base_url: String,
}

impl AppClient {
    /// 创建新的App API客户端实例
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http_client,
            base_url: "https://app-api.pixiv.net".to_string(),
        }
    }

    /// 设置API基础URL
    pub fn set_base_url(&mut self, url: String) {
        self.base_url = url;
    }

    /// 获取API基础URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 获取插画详情
    /// 
    /// # 参数
    /// * `illust_id` - 插画ID
    /// 
    /// # 返回
    /// 返回插画详情信息
    /// 
    /// # 示例
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

    /// 获取插画排行榜
    /// 
    /// # 参数
    /// * `mode` - 排行榜模式
    /// * `filter` - 过滤器
    /// * `date` - 日期 (格式: YYYY-MM-DD)
    /// * `offset` - 偏移量
    /// 
    /// # 返回
    /// 返回排行榜响应
    /// 
    /// # 示例
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

    /// 获取推荐插画
    /// 
    /// # 参数
    /// * `content_type` - 内容类型
    /// * `include_ranking_label` - 是否包含排行榜标签
    /// * `filter` - 过滤器
    /// * `max_bookmark_id_for_recommend` - 推荐的最大收藏ID
    /// * `min_bookmark_id_for_recent_illust` - 最近插画的最小收藏ID
    /// * `offset` - 偏移量
    /// * `include_ranking_illusts` - 是否包含排行榜插画
    /// * `bookmark_illust_ids` - 收藏的插画ID列表
    /// * `viewed` - 已查看的插画ID列表
    /// 
    /// # 返回
    /// 返回推荐响应
    /// 
    /// # 示例
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
        
        let response = self
            .http_client
            .send_request(reqwest::Method::GET, &url, Some(&params))
            .await?;
        
        let text = response.text().await?;
        let recommended: RecommendedResponse = serde_json::from_str(&text)?;
        
        Ok(recommended)
    }

    /// 搜索插画
    /// 
    /// # 参数
    /// * `word` - 搜索关键词
    /// * `search_target` - 搜索目标
    /// * `sort` - 排序方式
    /// * `duration` - 搜索持续时间
    /// * `start_date` - 开始日期 (格式: YYYY-MM-DD)
    /// * `end_date` - 结束日期 (格式: YYYY-MM-DD)
    /// * `filter` - 过滤器
    /// * `search_ai_type` - AI类型 (0: 过滤AI生成作品, 1: 显示AI生成作品)
    /// * `offset` - 偏移量
    /// 
    /// # 返回
    /// 返回搜索响应
    /// 
    /// # 示例
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

    /// 获取关注用户的插画
    ///
    /// # 参数
    /// * `restrict` - 关注限制 (公开/私密)
    /// * `offset` - 偏移量
    ///
    /// # 返回
    /// 返回关注用户的插画响应
    ///
    /// # 示例
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

    /// 获取插画评论
    ///
    /// # 参数
    /// * `illust_id` - 插画ID
    /// * `offset` - 偏移量
    /// * `include_total_comments` - 是否包含总评论数
    ///
    /// # 返回
    /// 返回评论响应
    ///
    /// # 示例
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

    /// 获取用户关注列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `restrict` - 关注限制 (公开/私密)
    /// * `offset` - 偏移量
    ///
    /// # 返回
    /// 返回用户关注响应
    ///
    /// # 示例
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

    /// 获取用户粉丝列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `filter` - 过滤器
    /// * `offset` - 偏移量
    ///
    /// # 返回
    /// 返回用户粉丝响应
    ///
    /// # 示例
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

    /// 获取用户好P友列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `offset` - 偏移量
    ///
    /// # 返回
    /// 返回用户好P友响应
    ///
    /// # 示例
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

    /// 添加插画收藏
    ///
    /// # 参数
    /// * `illust_id` - 插画ID
    /// * `restrict` - 收藏限制 (公开/私密)
    /// * `tags` - 标签列表
    ///
    /// # 返回
    /// 返回收藏响应
    ///
    /// # 示例
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

    /// 删除插画收藏
    ///
    /// # 参数
    /// * `illust_id` - 插画ID
    ///
    /// # 返回
    /// 返回收藏响应
    ///
    /// # 示例
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

    /// 获取趋势标签
    ///
    /// # 参数
    /// * `filter` - 过滤器
    ///
    /// # 返回
    /// 返回趋势标签响应
    ///
    /// # 示例
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

    /// 获取Ugoira元数据
    ///
    /// # 参数
    /// * `illust_id` - 插画ID
    ///
    /// # 返回
    /// 返回Ugoira元数据响应
    ///
    /// # 示例
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