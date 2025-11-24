use serde::Deserialize;
use std::collections::HashMap;

/// Follow restriction type
#[derive(Debug, Clone, Copy)]
pub enum FollowRestrict {
    /// Public
    Public,
    /// Private
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

/// Comment access control
#[derive(Deserialize, Debug, Clone)]
pub struct CommentAccessControl {
    /// Whether comments are allowed
    pub allow: bool,
}

/// Restriction attributes
#[derive(Deserialize, Debug, Clone)]
pub struct RestrictionAttributes {
    /// Restriction type
    #[serde(rename = "type")]
    pub restriction_type: String,
    /// Restriction value
    pub value: String,
}

/// Comment
#[derive(Deserialize, Debug, Clone)]
pub struct Comment {
    /// Comment ID
    pub id: u64,
    /// Comment content
    pub comment: String,
    /// Comment date
    pub date: String,
    /// Comment user
    pub user: Option<User>,
    /// Parent comment
    pub parent_comment: Option<Box<Comment>>,
}

/// Comment response
#[derive(Deserialize, Debug, Clone)]
pub struct CommentsResponse {
    /// Comment list
    pub comments: Vec<Comment>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Total comment count
    pub total_comments: Option<u32>,
}

/// Illustration follow response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustFollowResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Illustration bookmark response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustBookmarkResponse {
    /// Success status
    pub success: bool,
    /// Error message
    pub error: Option<String>,
}

/// Trending tags response
#[derive(Deserialize, Debug, Clone)]
pub struct TrendingTagsResponse {
    /// Trending tags list
    pub trend_tags: Vec<TrendingTag>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Trending tag
#[derive(Deserialize, Debug, Clone)]
pub struct TrendingTag {
    /// Tag name
    pub tag: String,
    /// Translated tag name
    pub translated_name: Option<String>,
    /// Illustration list
    pub illust: Option<Vec<Illust>>,
}

/// Ugoira metadata response
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraMetadataResponse {
    /// Ugoira metadata
    pub ugoira_metadata: UgoiraMetadata,
}

/// Ugoira metadata
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraMetadata {
    /// Illustration ID
    pub illust_id: u64,
    /// Frame delay list
    pub frames: Vec<UgoiraFrame>,
    /// MIME type
    pub mime_type: String,
    /// Original file URL
    pub original_src: String,
    /// File size
    pub zip_urls: ZipUrls,
}

/// Ugoira frame
#[derive(Deserialize, Debug, Clone)]
pub struct UgoiraFrame {
    /// File name
    pub file: String,
    /// Delay time (milliseconds)
    pub delay: u32,
}

/// ZIP file URLs
#[derive(Deserialize, Debug, Clone)]
pub struct ZipUrls {
    /// Medium size ZIP file URL
    pub medium: String,
    /// Large size ZIP file URL
    pub large: String,
    /// Original size ZIP file URL
    pub original: String,
}

/// User following response
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowingResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User followers response
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowerResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Illustration related response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustRelatedResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Illustration bookmark detail response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustBookmarkDetailResponse {
    /// User bookmark illust
    pub user_bookmark_illust: UserBookmarkIllust,
}

/// User bookmark illust
#[derive(Deserialize, Debug, Clone)]
pub struct UserBookmarkIllust {
    /// Bookmark ID
    pub id: u64,
    /// Is private
    pub is_private: bool,
    /// Tag list
    pub tags: Vec<Tag>,
    /// Restrict
    pub restrict: String,
}

/// Illustration new response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustNewResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User related response
#[derive(Deserialize, Debug, Clone)]
pub struct UserRelatedResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User recommended response
#[derive(Deserialize, Debug, Clone)]
pub struct UserRecommendedResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User list response
#[derive(Deserialize, Debug, Clone)]
pub struct UserListResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User follow delete response
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowDeleteResponse {
    /// Success status
    pub success: bool,
    /// Error message
    pub error: Option<String>,
}

/// User bookmark tags illust response
#[derive(Deserialize, Debug, Clone)]
pub struct UserBookmarkTagsIllustResponse {
    /// Bookmark tags list
    pub bookmark_tags: Vec<BookmarkTag>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Bookmark tag
#[derive(Deserialize, Debug, Clone)]
pub struct BookmarkTag {
    /// Tag name
    pub name: String,
    /// Translated tag name
    pub translated_name: Option<String>,
    /// Is private
    pub is_private: bool,
    /// Illusts
    pub illusts: Option<Vec<Illust>>,
}

/// User edit AI show settings response
#[derive(Deserialize, Debug, Clone)]
pub struct UserEditAiShowSettingsResponse {
    /// Success status
    pub success: bool,
    /// Error message
    pub error: Option<String>,
}

/// Search user response
#[derive(Deserialize, Debug, Clone)]
pub struct SearchUserResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User follow add response
#[derive(Deserialize, Debug, Clone)]
pub struct UserFollowAddResponse {
    /// Success status
    pub success: bool,
    /// Error message
    pub error: Option<String>,
}

/// User illustrations response
#[derive(Deserialize, Debug, Clone)]
pub struct UserIllustrationsResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User mypixiv response
#[derive(Deserialize, Debug, Clone)]
pub struct UserMypixivResponse {
    /// User preview list
    pub user_previews: Vec<UserPreview>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// User preview
#[derive(Deserialize, Debug, Clone)]
pub struct UserPreview {
    /// User information
    pub user: User,
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Novel list
    pub novels: Option<Vec<Novel>>,
    /// Is muted
    pub is_muted: Option<bool>,
}

/// Novel information
#[derive(Deserialize, Debug, Clone)]
pub struct Novel {
    /// Novel ID
    pub id: u64,
    /// Novel title
    pub title: String,
    /// Novel type
    #[serde(rename = "type")]
    pub novel_type: String,
    /// Novel description
    pub caption: String,
    /// Restriction level
    pub restrict: u32,
    /// User information
    pub user: User,
    /// Tag list
    pub tags: Vec<Tag>,
    /// Creation date
    pub create_date: String,
    /// Page count
    pub page_count: u32,
    /// Text length
    pub text_length: u32,
    /// Series information
    pub series: Option<Series>,
    /// Total views
    pub total_view: u64,
    /// Total bookmarks
    pub total_bookmarks: u64,
    /// Is bookmarked
    pub is_bookmarked: bool,
    /// Is visible
    pub visible: bool,
    /// Is muted
    pub is_muted: bool,
    /// Is mypixiv only
    pub is_mypixiv_only: bool,
    /// Is X restricted
    pub is_x_restricted: bool,
    /// Novel AI type
    pub novel_ai_type: u32,
    /// Comment access control
    pub comment_access_control: Option<CommentAccessControl>,
}

/// Image URLs collection
#[derive(Deserialize, Debug, Clone)]
pub struct ImageUrls {
    /// Medium square image URL
    pub square_medium: String,
    /// Medium image URL
    pub medium: String,
    /// Large image URL
    pub large: String,
}

/// User avatar URL
#[derive(Deserialize, Debug, Clone)]
pub struct ProfileImageUrls {
    /// Medium avatar URL
    pub medium: String,
}

/// User information
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// User ID
    pub id: u64,
    /// Username
    pub name: String,
    /// User account
    pub account: String,
    /// User avatar URL
    pub profile_image_urls: ProfileImageUrls,
    /// Comment
    pub comment: Option<String>,
    /// Is followed
    pub is_followed: Option<bool>,
}

/// Illustration tag
#[derive(Deserialize, Debug, Clone)]
pub struct Tag {
    /// Tag name
    pub name: String,
    /// Translated tag name
    pub translated_name: Option<String>,
}

/// Single page metadata
#[derive(Deserialize, Debug, Clone)]
pub struct MetaSinglePage {
    /// Original image URL
    pub original_image_url: Option<String>,
}

/// Page metadata
#[derive(Deserialize, Debug, Clone)]
pub struct MetaPage {
    /// Image URLs
    pub image_urls: ImageUrls,
}

/// Series information
#[derive(Deserialize, Debug, Clone)]
pub struct Series {
    /// Series ID
    pub id: u64,
    /// Series title
    pub title: String,
}

/// Illustration information
#[derive(Deserialize, Debug, Clone)]
pub struct Illust {
    /// Illustration ID
    pub id: u64,
    /// Illustration title
    pub title: String,
    /// Illustration type
    #[serde(rename = "type")]
    pub illust_type: String,
    /// Image URLs
    pub image_urls: ImageUrls,
    /// Illustration description
    pub caption: String,
    /// Restriction level
    pub restrict: u32,
    /// User information
    pub user: User,
    /// Tag list
    pub tags: Vec<Tag>,
    /// Tools used
    pub tools: Vec<String>,
    /// Creation date
    pub create_date: String,
    /// Page count
    pub page_count: u32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Sanity level
    pub sanity_level: u32,
    /// R-18 level
    pub x_restrict: u32,
    /// Series information
    pub series: Option<Series>,
    /// Single page metadata
    pub meta_single_page: MetaSinglePage,
    /// Page metadata
    pub meta_pages: Vec<MetaPage>,
    /// Total views
    #[serde(rename = "total_view_count")]
    pub total_view: u64,
    /// Total bookmarks
    #[serde(rename = "total_bookmarks_count")]
    pub total_bookmarks: u64,
    /// Is bookmarked
    pub is_bookmarked: bool,
    /// Is visible
    pub visible: bool,
    /// Is muted
    pub is_muted: bool,
    /// AI type
    pub illust_ai_type: u32,
    /// Illustration book style
    pub illust_book_style: u32,
    /// Total comments
    pub total_comments: Option<u32>,
    /// Comment access control
    #[serde(default)]
    pub comment_access_control: Option<CommentAccessControl>,
    /// Restriction attributes
    #[serde(default)]
    pub restriction_attributes: Option<Vec<RestrictionAttributes>>,
}

/// Illustration detail response
#[derive(Deserialize, Debug, Clone)]
pub struct IllustDetail {
    /// Illustration information
    pub illust: Illust,
}

/// Ranking response
#[derive(Deserialize, Debug, Clone)]
pub struct RankingResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Ranking date
    pub date: Option<String>,
    /// Ranking mode
    pub mode: Option<String>,
}

/// Recommendation response
#[derive(Deserialize, Debug, Clone)]
pub struct RecommendedResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Ranking label
    pub ranking_label: Option<RankingLabel>,
}

/// Ranking label
#[derive(Deserialize, Debug, Clone)]
pub struct RankingLabel {
    /// Label text
    pub text: String,
    /// Label type
    #[serde(rename = "type")]
    pub label_type: String,
}

/// Search illustration response
#[derive(Deserialize, Debug, Clone)]
pub struct SearchIllustResponse {
    /// Illustration list
    pub illusts: Vec<Illust>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Search span limit
    pub search_span_limit: u32,
    /// Show AI works
    pub show_ai: bool,
}

/// Search target type
#[derive(Debug, Clone, Copy)]
pub enum SearchTarget {
    /// Partial match for tags
    PartialMatchForTags,
    /// Exact match for tags
    ExactMatchForTags,
    /// Title and caption
    TitleAndCaption,
    /// Keyword
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

/// Sort method
#[derive(Debug, Clone, Copy)]
pub enum Sort {
    /// Date descending
    DateDesc,
    /// Date ascending
    DateAsc,
    /// Popular descending
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

/// Ranking mode
#[derive(Debug, Clone, Copy)]
pub enum RankingMode {
    /// Daily ranking
    Day,
    /// Weekly ranking
    Week,
    /// Monthly ranking
    Month,
    /// Daily male ranking
    DayMale,
    /// Daily female ranking
    DayFemale,
    /// Weekly original ranking
    WeekOriginal,
    /// Weekly rookie ranking
    WeekRookie,
    /// Daily manga ranking
    DayManga,
    /// Daily R-18 ranking
    DayR18,
    /// Daily R-18 male ranking
    DayMaleR18,
    /// Daily R-18 female ranking
    DayFemaleR18,
    /// Weekly R-18 ranking
    WeekR18,
    /// Weekly R-18G ranking
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

/// Content type
#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    /// Illustration
    Illust,
    /// Manga
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

/// Filter type
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    /// iOS filter
    ForIOS,
    /// No filter
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

/// Search duration
#[derive(Debug, Clone, Copy)]
pub enum Duration {
    /// Within last day
    WithinLastDay,
    /// Within last week
    WithinLastWeek,
    /// Within last month
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

/// User novels response
#[derive(Deserialize, Debug, Clone)]
pub struct UserNovelsResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Novel series detail
#[derive(Deserialize, Debug, Clone)]
pub struct NovelSeriesDetail {
    /// Series ID
    pub id: u64,
    /// Series title
    pub title: String,
    /// Series caption
    pub caption: String,
    /// User information
    pub user: User,
    /// Tag list
    pub tags: Vec<Tag>,
    /// Creation date
    pub create_date: String,
    /// Page count
    pub page_count: u32,
    /// Text length
    pub text_length: u32,
    /// Total series
    pub total_series: u32,
    /// Total novels
    pub total_novels: u32,
    /// Total views
    pub total_view: u64,
    /// Total bookmarks
    pub total_bookmarks: u64,
    /// Is bookmarked
    pub is_bookmarked: bool,
    /// Is visible
    pub visible: bool,
    /// Is muted
    pub is_muted: bool,
    /// Is mypixiv only
    pub is_mypixiv_only: bool,
    /// Is X restricted
    pub is_x_restricted: bool,
    /// Novel AI type
    pub novel_ai_type: u32,
    /// Comment access control
    pub comment_access_control: Option<CommentAccessControl>,
}

/// Novel series response
#[derive(Deserialize, Debug, Clone)]
pub struct NovelSeriesResponse {
    /// Novel series detail
    pub novel_series_detail: NovelSeriesDetail,
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Novel new response
#[derive(Deserialize, Debug, Clone)]
pub struct NovelNewResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Novel follow response
#[derive(Deserialize, Debug, Clone)]
pub struct NovelFollowResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Novel recommended response
#[derive(Deserialize, Debug, Clone)]
pub struct NovelRecommendedResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Ranking label
    pub ranking_label: Option<RankingLabel>,
}

/// Search novel response
#[derive(Deserialize, Debug, Clone)]
pub struct SearchNovelResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
    /// Search span limit
    pub search_span_limit: u32,
    /// Show AI works
    pub show_ai: bool,
}

/// User bookmarks novel response
#[derive(Deserialize, Debug, Clone)]
pub struct UserBookmarksNovelResponse {
    /// Novel list
    pub novels: Vec<Novel>,
    /// Next page URL
    pub next_url: Option<String>,
}

/// Webview novel response
#[derive(Deserialize, Debug, Clone)]
pub struct WebviewNovelResponse {
    /// Novel information
    pub novel: Novel,
    /// Novel text
    pub novel_text: String,
}

/// Search target type for novels
#[derive(Debug, Clone, Copy)]
pub enum NovelSearchTarget {
    /// Partial match for tags
    PartialMatchForTags,
    /// Exact match for tags
    ExactMatchForTags,
    /// Text
    Text,
    /// Keyword
    Keyword,
}

impl ToString for NovelSearchTarget {
    fn to_string(&self) -> String {
        match self {
            NovelSearchTarget::PartialMatchForTags => "partial_match_for_tags".to_string(),
            NovelSearchTarget::ExactMatchForTags => "exact_match_for_tags".to_string(),
            NovelSearchTarget::Text => "text".to_string(),
            NovelSearchTarget::Keyword => "keyword".to_string(),
        }
    }
}

/// Follow restriction type for novels
#[derive(Debug, Clone, Copy)]
pub enum NovelFollowRestrict {
    /// Public
    Public,
    /// Private
    Private,
    /// All
    All,
}

impl ToString for NovelFollowRestrict {
    fn to_string(&self) -> String {
        match self {
            NovelFollowRestrict::Public => "public".to_string(),
            NovelFollowRestrict::Private => "private".to_string(),
            NovelFollowRestrict::All => "all".to_string(),
        }
    }
}