use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 公开API插画信息
#[derive(Deserialize, Debug, Clone)]
pub struct PublicIllust {
    /// 插画ID
    pub id: u64,
    /// 插画标题
    pub title: String,
    /// 插画类型 (0=插画, 1=漫画, 2=动图)
    #[serde(rename = "illust_type")]
    pub illust_type: u8,
    /// 插画标签
    pub tags: Vec<String>,
    /// 插画说明
    pub caption: String,
    /// 插画尺寸信息
    pub image_urls: ImageUrls,
    /// 用户信息
    pub user: PublicUser,
    /// 创建时间
    pub create_date: String,
    /// 统计信息
    pub stats: Option<Stats>,
    /// 是否为R18内容
    pub r18: bool,
    /// 总页面数
    pub page_count: u32,
    /// 书签数
    pub bookmark_count: Option<u32>,
    /// 评论数
    pub comment_count: Option<u32>,
    /// 点击数
    pub view_count: Option<u32>,
}

/// 图片URL集合
#[derive(Deserialize, Debug, Clone)]
pub struct ImageUrls {
    /// 小图URL
    pub square_medium: Option<String>,
    /// 中图URL
    pub medium: Option<String>,
    /// 大图URL
    pub large: Option<String>,
    /// 原图URL
    pub original: Option<String>,
}

/// 公开API用户信息
#[derive(Deserialize, Debug, Clone)]
pub struct PublicUser {
    /// 用户ID
    pub id: u64,
    /// 用户名
    pub name: String,
    /// 用户头像URL
    pub profile_image_urls: ProfileImageUrls,
    /// 是否关注
    pub is_followed: Option<bool>,
}

/// 头像URL集合
#[derive(Deserialize, Debug, Clone)]
pub struct ProfileImageUrls {
    /// 头像URL
    pub px_16x16: Option<String>,
    pub px_50x50: Option<String>,
    pub px_170x170: Option<String>,
}

/// 统计信息
#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    /// 书签数
    pub bookmarks_count: u32,
    /// 评论数
    pub comments_count: u32,
    /// 点击数
    pub views_count: u32,
    /// 好评数
    pub score: u32,
}

/// 公开API搜索响应
#[derive(Deserialize, Debug)]
pub struct PublicSearchResponse {
    /// 搜索结果列表
    pub illusts: Vec<PublicIllust>,
    /// 下一页URL（如果有）
    pub next_url: Option<String>,
    /// 搜索元数据
    pub search_meta: Option<SearchMeta>,
}

/// 搜索元数据
#[derive(Deserialize, Debug)]
pub struct SearchMeta {
    /// 搜索ID
    pub search_id: String,
    /// 搜索时间
    pub time: u64,
}

/// 公开API用户详情响应
#[derive(Deserialize, Debug)]
pub struct PublicUserDetail {
    /// 用户详情
    pub user: PublicUser,
    /// 用户作品列表
    pub illusts: HashMap<String, PublicIllust>,
    /// 用户漫画列表
    pub manga: HashMap<String, PublicIllust>,
    /// 用户小说列表
    pub novels: HashMap<String, PublicNovel>,
    /// 用户信息
    pub profile: UserProfile,
    /// 用户统计信息
    pub profile_publicity: UserProfilePublicity,
    /// 用户工作信息
    pub workspace: Workspace,
}

/// 用户小说信息
#[derive(Deserialize, Debug)]
pub struct PublicNovel {
    /// 小说ID
    pub id: u64,
    /// 小说标题
    pub title: String,
    /// 小说标签
    pub tags: Vec<String>,
    /// 小说说明
    pub caption: String,
    /// 小说封面URL
    pub image_urls: ImageUrls,
    /// 作者信息
    pub user: PublicUser,
    /// 创建时间
    pub create_date: String,
    /// 文字数
    pub text_length: u32,
    /// 总页数
    pub page_count: u32,
    /// 可视化页数
    pub visible_page_count: u32,
    /// 是否为R18内容
    pub is_muted: bool,
}

/// 用户信息
#[derive(Deserialize, Debug)]
pub struct UserProfile {
    /// 生日
    pub birth: String,
    /// 生日公开度
    pub birth_day: String,
    /// 生日类型
    pub birth_year: i32,
    /// 性别
    pub gender: String,
    /// 地区
    pub region: String,
    /// 位置
    pub address_id: u32,
    /// 职业
    pub country_code: String,
    /// 职业ID
    pub job: String,
    /// 职业ID
    pub job_id: u32,
    /// 兴趣
    pub interests: String,
    /// 海报URL
    pub comment: Option<String>,
    /// 作品URL
    pub webpage: Option<String>,
    /// Twitter用户名
    pub twitter_account: Option<String>,
    /// Twitter URL
    pub twitter_url: Option<String>,
    /// 背景图片URL
    pub pawoo_url: Option<String>,
    /// 是否为Pawoo用户
    pub is_pawoo_user: Option<bool>,
}

/// 用户信息公开度
#[derive(Deserialize, Debug)]
pub struct UserProfilePublicity {
    /// 生日公开度
    pub birth_day: String,
    /// 性别公开度
    pub gender: String,
    /// 地区公开度
    pub region: String,
    /// 职业公开度
    pub job: String,
    /// 兴趣公开度
    pub interests: String,
    /// 作品公开度
    pub comment: String,
    /// 网站公开度
    pub webpage: String,
    /// Twitter公开度
    pub twitter: String,
    /// Pawoo公开度
    pub pawoo: String,
}

/// 工作空间信息
#[derive(Deserialize, Debug)]
pub struct Workspace {
    /// 桌子
    pub desk: Option<String>,
    /// 椅子
    pub chair: Option<String>,
    /// 操作系统
    pub operating_system: Option<String>,
    /// 工具
    pub tool: Option<String>,
    /// 评论
    pub comment: Option<String>,
    /// 音乐
    pub music: Option<String>,
    /// 茶
    pub tea: Option<String>,
    /// 照片URL
    pub pc: Option<String>,
    pub monitor: Option<String>,
    pub software: Option<String>,
    pub scanner: Option<String>,
    pub tablet: Option<String>,
    pub mouse: Option<String>,
    pub printer: Option<String>,
    pub desktop: Option<String>,
    pub music_player: Option<String>,
    pub router: Option<String>,
    pub other: Option<String>,
    pub camera: Option<String>,
}

/// 公开API用户作品响应
#[derive(Deserialize, Debug)]
pub struct PublicUserIllusts {
    /// 用户作品列表
    pub illusts: Vec<PublicIllust>,
    /// 下一页URL（如果有）
    pub next_url: Option<String>,
}

/// 公开API用户收藏响应
#[derive(Deserialize, Debug)]
pub struct PublicUserBookmarks {
    /// 收藏作品列表
    pub illusts: Vec<PublicIllust>,
    /// 下一页URL（如果有）
    pub next_url: Option<String>,
}

/// 搜索目标类型
#[derive(Debug, Clone)]
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

impl std::fmt::Display for SearchTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchTarget::PartialMatchForTags => write!(f, "partial_match_for_tags"),
            SearchTarget::ExactMatchForTags => write!(f, "exact_match_for_tags"),
            SearchTarget::TitleAndCaption => write!(f, "title_and_caption"),
            SearchTarget::Keyword => write!(f, "keyword"),
        }
    }
}

/// 排序方式
#[derive(Debug, Clone)]
pub enum Sort {
    /// 日期降序
    DateDesc,
    /// 日期升序
    DateAsc,
    /// 热门降序
    PopularDesc,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::DateDesc => write!(f, "date_desc"),
            Sort::DateAsc => write!(f, "date_asc"),
            Sort::PopularDesc => write!(f, "popular_desc"),
        }
    }
}

/// 内容类型
#[derive(Debug, Clone)]
pub enum ContentType {
    /// 插画
    Illust,
    /// 漫画
    Manga,
    /// 动图
    Ugoira,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Illust => write!(f, "illust"),
            ContentType::Manga => write!(f, "manga"),
            ContentType::Ugoira => write!(f, "ugoira"),
        }
    }
}

/// 过滤器类型
#[derive(Debug, Clone)]
pub enum Filter {
    /// iOS过滤器
    ForIOS,
    /// 无过滤器
    None,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::ForIOS => write!(f, "for_ios"),
            Filter::None => write!(f, ""),
        }
    }
}

/// 搜索持续时间
#[derive(Debug, Clone)]
pub enum Duration {
    /// 最近一天
    WithinLastDay,
    /// 最近一周
    WithinLastWeek,
    /// 最近一月
    WithinLastMonth,
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Duration::WithinLastDay => write!(f, "within_last_day"),
            Duration::WithinLastWeek => write!(f, "within_last_week"),
            Duration::WithinLastMonth => write!(f, "within_last_month"),
        }
    }
}

/// 访问限制类型
#[derive(Debug, Clone)]
pub enum Restrict {
    Public,  // 公开
    Private, // 私密
}

impl std::fmt::Display for Restrict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Restrict::Public => write!(f, "public"),
            Restrict::Private => write!(f, "private"),
        }
    }
}