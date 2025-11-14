use serde::Deserialize;
use std::collections::HashMap;

/// 关注限制类型
#[derive(Debug, Clone, Copy)]
pub enum FollowRestrict {
    /// 公开
    Public,
    /// 私密
    Private,
}

impl ToString for FollowRestrict {
    fn to_string(&self) -> String {
        match self {
            FollowRestrict::Public => "public".to_string(),
            FollowRestrict::Private => "private".to_string(),
        }
    }
}

/// 评论访问控制
#[derive(Deserialize, Debug, Clone)]
pub struct CommentAccessControl {
    /// 是否允许评论
    pub allow: bool,
}

/// 限制属性
#[derive(Deserialize, Debug, Clone)]
pub struct RestrictionAttributes {
    /// 限制类型
    #[serde(rename = "type")]
    pub restriction_type: String,
    /// 限制值
    pub value: String,
}

/// 评论
#[derive(Deserialize, Debug, Clone)]
pub struct Comment {
    /// 评论ID
    pub id: u64,
    /// 评论内容
    pub comment: String,
    /// 评论日期
    pub date: String,
    /// 评论用户
    pub user: Option<User>,
    /// 父评论
    pub parent_comment: Option<Box<Comment>>,
}

/// 评论响应
#[derive(Deserialize, Debug, Clone)]
pub struct CommentsResponse {
    /// 评论列表
    pub comments: Vec<Comment>,
    /// 下一页URL
    pub next_url: Option<String>,
    /// 总评论数
    pub total_comments: Option<u32>,
}

/// 插画关注响应
#[derive(Deserialize, Debug, Clone)]
pub struct IllustFollowResponse {
    /// 插画列表
    pub illusts: Vec<Illust>,
    /// 下一页URL
    pub next_url: Option<String>,
}

/// 插画收藏响应
#[derive(Deserialize, Debug, Clone)]
pub struct IllustBookmarkResponse {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

/// 趋势标签响应
#[derive(Deserialize, Debug, Clone)]
pub struct TrendingTagsResponse {
    /// 趋势标签列表
    pub trend_tags: Vec<TrendingTag>,
    /// 下一页URL
    pub next_url: Option<String>,
}

/// 趋势标签
#[derive(Deserialize, Debug, Clone)]
pub struct TrendingTag {
    /// 标签名
    pub tag: String,
    /// 翻译后的标签名
    pub translated_name: Option<String>,
    /// 插画列表
    pub illust: Option<Vec<Illust>>,
}

/// Ugoira元数据响应
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraMetadataResponse {
    /// Ugoira元数据
    pub ugoira_metadata: UgoiraMetadata,
}

/// Ugoira元数据
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraMetadata {
    /// 插画ID
    pub illust_id: u64,
    /// 帧延迟列表
    pub frames: Vec<UgoiraFrame>,
    /// MIME类型
    pub mime_type: String,
    /// 原始文件URL
    pub original_src: String,
    /// 文件大小
    pub zip_urls: ZipUrls,
}

/// Ugoira帧
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraFrame {
    /// 文件名
    pub file: String,
    /// 延迟时间（毫秒）
    pub delay: u32,
}

/// ZIP文件URL
#[derive(Deserialize, Debug, Clone)]
pub struct ZipUrls {
    /// 中等大小ZIP文件URL
    pub medium: String,
    /// 大尺寸ZIP文件URL
    pub large: String,
    /// 原始大小ZIP文件URL
    pub original: String,
}

/// 用户关注响应
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowingResponse {
    /// 用户预览列表
    pub user_previews: Vec<UserPreview>,
    /// 下一页URL
    pub next_url: Option<String>,
}

/// 用户粉丝响应
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowerResponse {
    /// 用户预览列表
    pub user_previews: Vec<UserPreview>,
    /// 下一页URL
    pub next_url: Option<String>,
}

/// 用户好P友响应
#[derive(Deserialize, Debug, Clone)]
pub struct UserMypixivResponse {
    /// 用户预览列表
    pub user_previews: Vec<UserPreview>,
    /// 下一页URL
    pub next_url: Option<String>,
}

/// 用户预览
#[derive(Deserialize, Debug, Clone)]
pub struct UserPreview {
    /// 用户信息
    pub user: User,
    /// 插画列表
    pub illusts: Vec<Illust>,
    /// 小说列表
    pub novels: Option<Vec<Novel>>,
    /// 是否被静音
    pub is_muted: Option<bool>,
}

/// 小说信息
#[derive(Deserialize, Debug, Clone)]
pub struct Novel {
    /// 小说ID
    pub id: u64,
    /// 小说标题
    pub title: String,
    /// 小说类型
    #[serde(rename = "type")]
    pub novel_type: String,
    /// 小说说明
    pub caption: String,
    /// 限制级别
    pub restrict: u32,
    /// 用户信息
    pub user: User,
    /// 标签列表
    pub tags: Vec<Tag>,
    /// 创建日期
    pub create_date: String,
    /// 页数
    pub page_count: u32,
    /// 文本长度
    pub text_length: u32,
    /// 系列信息
    pub series: Option<Series>,
    /// 总浏览数
    pub total_view: u64,
    /// 总收藏数
    pub total_bookmarks: u64,
    /// 是否已收藏
    pub is_bookmarked: bool,
    /// 是否可见
    pub visible: bool,
    /// 是否被静音
    pub is_muted: bool,
    /// 是否仅限好P友
    pub is_mypixiv_only: bool,
    /// 是否X限制
    pub is_x_restricted: bool,
    /// 小说AI类型
    pub novel_ai_type: u32,
    /// 评论访问控制
    pub comment_access_control: Option<CommentAccessControl>,
}

/// 图片URL集合
#[derive(Deserialize, Debug, Clone)]
pub struct ImageUrls {
    /// 中等方形图片URL
    pub square_medium: String,
    /// 中等图片URL
    pub medium: String,
    /// 大图片URL
    pub large: String,
}

/// 用户头像URL
#[derive(Deserialize, Debug, Clone)]
pub struct ProfileImageUrls {
    /// 中等头像URL
    pub medium: String,
}

/// 用户信息
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// 用户ID
    pub id: u64,
    /// 用户名
    pub name: String,
    /// 用户账号
    pub account: String,
    /// 用户头像URL
    pub profile_image_urls: ProfileImageUrls,
    /// 评论
    pub comment: Option<String>,
    /// 是否已关注
    pub is_followed: Option<bool>,
}

/// 插画标签
#[derive(Deserialize, Debug, Clone)]
pub struct Tag {
    /// 标签名
    pub name: String,
    /// 翻译后的标签名
    pub translated_name: Option<String>,
}

/// 单页元数据
#[derive(Deserialize, Debug, Clone)]
pub struct MetaSinglePage {
    /// 原始图片URL
    pub original_image_url: Option<String>,
}

/// 页面元数据
#[derive(Deserialize, Debug, Clone)]
pub struct MetaPage {
    /// 图片URL
    pub image_urls: ImageUrls,
}

/// 系列信息
#[derive(Deserialize, Debug, Clone)]
pub struct Series {
    /// 系列ID
    pub id: u64,
    /// 系列标题
    pub title: String,
}

/// 插画信息
#[derive(Deserialize, Debug, Clone)]
pub struct Illust {
    /// 插画ID
    pub id: u64,
    /// 插画标题
    pub title: String,
    /// 插画类型
    #[serde(rename = "type")]
    pub illust_type: String,
    /// 图片URL
    pub image_urls: ImageUrls,
    /// 插画说明
    pub caption: String,
    /// 限制级别
    pub restrict: u32,
    /// 用户信息
    pub user: User,
    /// 标签列表
    pub tags: Vec<Tag>,
    /// 使用的工具
    pub tools: Vec<String>,
    /// 创建日期
    pub create_date: String,
    /// 页数
    pub page_count: u32,
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 审查级别
    pub sanity_level: u32,
    /// R-18级别
    pub x_restrict: u32,
    /// 系列信息
    pub series: Option<Series>,
    /// 单页元数据
    pub meta_single_page: MetaSinglePage,
    /// 页面元数据
    pub meta_pages: Vec<MetaPage>,
    /// 总浏览数
    #[serde(rename = "total_view_count")]
    pub total_view: u64,
    /// 总收藏数
    #[serde(rename = "total_bookmarks_count")]
    pub total_bookmarks: u64,
    /// 是否已收藏
    pub is_bookmarked: bool,
    /// 是否可见
    pub visible: bool,
    /// 是否被静音
    pub is_muted: bool,
    /// AI类型
    pub illust_ai_type: u32,
    /// 插画书籍风格
    pub illust_book_style: u32,
    /// 总评论数
    pub total_comments: Option<u32>,
    /// 评论访问控制
    #[serde(default)]
    pub comment_access_control: Option<CommentAccessControl>,
    /// 限制属性
    #[serde(default)]
    pub restriction_attributes: Option<Vec<RestrictionAttributes>>,
}

/// 插画详情响应
#[derive(Deserialize, Debug, Clone)]
pub struct IllustDetail {
    /// 插画信息
    pub illust: Illust,
}

/// 排行榜响应
#[derive(Deserialize, Debug, Clone)]
pub struct RankingResponse {
    /// 插画列表
    pub illusts: Vec<Illust>,
    /// 下一页URL
    pub next_url: Option<String>,
    /// 排行榜日期
    pub date: Option<String>,
    /// 排行榜模式
    pub mode: Option<String>,
}

/// 推荐响应
#[derive(Deserialize, Debug, Clone)]
pub struct RecommendedResponse {
    /// 插画列表
    pub illusts: Vec<Illust>,
    /// 下一页URL
    pub next_url: Option<String>,
    /// 排行榜标签
    pub ranking_label: Option<RankingLabel>,
}

/// 排行榜标签
#[derive(Deserialize, Debug, Clone)]
pub struct RankingLabel {
    /// 标签文本
    pub text: String,
    /// 标签类型
    #[serde(rename = "type")]
    pub label_type: String,
}

/// 搜索插画响应
#[derive(Deserialize, Debug, Clone)]
pub struct SearchIllustResponse {
    /// 插画列表
    pub illusts: Vec<Illust>,
    /// 下一页URL
    pub next_url: Option<String>,
    /// 搜索跨度限制
    pub search_span_limit: u32,
    /// 是否显示AI作品
    pub show_ai: bool,
}

/// 搜索目标类型
#[derive(Debug, Clone, Copy)]
pub enum SearchTarget {
    /// 标签部分匹配
    PartialMatchForTags,
    /// 标签完全匹配
    ExactMatchForTags,
    /// 标题和说明
    TitleAndCaption,
    /// 关键词
    Keyword,
}

impl ToString for SearchTarget {
    fn to_string(&self) -> String {
        match self {
            SearchTarget::PartialMatchForTags => "partial_match_for_tags".to_string(),
            SearchTarget::ExactMatchForTags => "exact_match_for_tags".to_string(),
            SearchTarget::TitleAndCaption => "title_and_caption".to_string(),
            SearchTarget::Keyword => "keyword".to_string(),
        }
    }
}

/// 排序方式
#[derive(Debug, Clone, Copy)]
pub enum Sort {
    /// 日期降序
    DateDesc,
    /// 日期升序
    DateAsc,
    /// 热门降序
    PopularDesc,
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::DateDesc => "date_desc".to_string(),
            Sort::DateAsc => "date_asc".to_string(),
            Sort::PopularDesc => "popular_desc".to_string(),
        }
    }
}

/// 排行榜模式
#[derive(Debug, Clone, Copy)]
pub enum RankingMode {
    /// 日榜
    Day,
    /// 周榜
    Week,
    /// 月榜
    Month,
    /// 男性日榜
    DayMale,
    /// 女性日榜
    DayFemale,
    /// 原创周榜
    WeekOriginal,
    /// 新人周榜
    WeekRookie,
    /// 漫画日榜
    DayManga,
    /// R-18日榜
    DayR18,
    /// R-18男性日榜
    DayMaleR18,
    /// R-18女性日榜
    DayFemaleR18,
    /// R-18周榜
    WeekR18,
    /// R-18G周榜
    WeekR18g,
}

impl ToString for RankingMode {
    fn to_string(&self) -> String {
        match self {
            RankingMode::Day => "day".to_string(),
            RankingMode::Week => "week".to_string(),
            RankingMode::Month => "month".to_string(),
            RankingMode::DayMale => "day_male".to_string(),
            RankingMode::DayFemale => "day_female".to_string(),
            RankingMode::WeekOriginal => "week_original".to_string(),
            RankingMode::WeekRookie => "week_rookie".to_string(),
            RankingMode::DayManga => "day_manga".to_string(),
            RankingMode::DayR18 => "day_r18".to_string(),
            RankingMode::DayMaleR18 => "day_male_r18".to_string(),
            RankingMode::DayFemaleR18 => "day_female_r18".to_string(),
            RankingMode::WeekR18 => "week_r18".to_string(),
            RankingMode::WeekR18g => "week_r18g".to_string(),
        }
    }
}

/// 内容类型
#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    /// 插画
    Illust,
    /// 漫画
    Manga,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Illust => "illust".to_string(),
            ContentType::Manga => "manga".to_string(),
        }
    }
}

/// 过滤器类型
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    /// iOS过滤器
    ForIOS,
    /// 无过滤器
    None,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match self {
            Filter::ForIOS => "for_ios".to_string(),
            Filter::None => "".to_string(),
        }
    }
}

/// 搜索持续时间
#[derive(Debug, Clone, Copy)]
pub enum Duration {
    /// 最近一天
    WithinLastDay,
    /// 最近一周
    WithinLastWeek,
    /// 最近一月
    WithinLastMonth,
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        match self {
            Duration::WithinLastDay => "within_last_day".to_string(),
            Duration::WithinLastWeek => "within_last_week".to_string(),
            Duration::WithinLastMonth => "within_last_month".to_string(),
        }
    }
}