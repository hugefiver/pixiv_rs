use crate::error::{PixivError, Result};
use crate::models::app::{
    ContentType, Duration, Filter, IllustDetail, RankingMode, RankingResponse, RecommendedResponse,
    SearchIllustResponse, SearchTarget, Sort,
};
use crate::network::bypass_sni::BypassSniClient;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::debug;

/// 绕过SNI的App API客户端，用于与Pixiv App API交互
///
/// 这个客户端通过SNI绕过功能访问Pixiv API，允许在某些网络受限环境下使用。
///
/// # 示例
///
/// ```rust
/// use pixiv_rs::client::bypass_sni::BypassSniAppClient;
///
/// // 使用IP地址创建SNI绕过客户端
/// let client = BypassSniAppClient::with_ip("210.140.131.145")?;
///
/// // 设置访问令牌后使用API
/// // client.http_client.set_access_token("your_access_token".to_string());
/// // let illust = client.illust_detail(12345).await?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct BypassSniAppClient {
    /// SNI绕过HTTP客户端
    http_client: BypassSniClient,
    /// API基础URL
    base_url: String,
}

impl BypassSniAppClient {
    /// 创建新的绕过SNI的App API客户端实例
    pub fn new(http_client: BypassSniClient) -> Self {
        Self {
            http_client,
            base_url: "https://app-api.pixiv.net".to_string(),
        }
    }

    /// 使用指定IP创建绕过SNI的App API客户端实例
    pub fn with_ip(ip: &str) -> Result<Self> {
        let http_client = BypassSniClient::new(ip)?;
        Ok(Self {
            http_client,
            base_url: "https://app-api.pixiv.net".to_string(),
        })
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
    /// let client = BypassSniAppClient::with_ip("210.140.131.145")?;
    /// let detail = client.illust_detail(12345678).await?;
    /// ```
    pub async fn illust_detail(&self, illust_id: u64) -> Result<IllustDetail> {
        debug!(illust_id = %illust_id, "Fetching illustration detail with SNI bypass");
        
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
    /// let client = BypassSniAppClient::with_ip("210.140.131.145")?;
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
            "Fetching illustration ranking with SNI bypass"
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
    /// let client = BypassSniAppClient::with_ip("210.140.131.145")?;
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
            "Fetching recommended illustrations with SNI bypass"
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
    /// let client = BypassSniAppClient::with_ip("210.140.131.145")?;
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
            "Searching illustrations with SNI bypass"
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bypass_sni_app_client_creation() {
        let result = BypassSniAppClient::with_ip("210.140.131.145");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_ip() {
        let result = BypassSniAppClient::with_ip("invalid_ip");
        assert!(result.is_err());
    }
}