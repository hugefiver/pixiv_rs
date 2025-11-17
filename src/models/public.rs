use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Public API illustration information
#[derive(Deserialize, Debug, Clone)]
pub struct PublicIllust {
    /// Illustration ID
    pub id: u64,
    /// Illustration title
    pub title: String,
    /// Illustration type (0=illustration, 1=manga, 2=ugoira)
    #[serde(rename = "illust_type")]
    pub illust_type: u8,
    /// Illustration tags
    pub tags: Vec<String>,
    /// Illustration description
    pub caption: String,
    /// Illustration size information
    pub image_urls: ImageUrls,
    /// User information
    pub user: PublicUser,
    /// Creation time
    pub create_date: String,
    /// Statistics information
    pub stats: Option<Stats>,
    /// Is R18 content
    pub r18: bool,
    /// Total page count
    pub page_count: u32,
    /// Bookmark count
    pub bookmark_count: Option<u32>,
    /// Comment count
    pub comment_count: Option<u32>,
    /// View count
    pub view_count: Option<u32>,
}

/// Image URLs collection
#[derive(Deserialize, Debug, Clone)]
pub struct ImageUrls {
    /// Small image URL
    pub square_medium: Option<String>,
    /// Medium image URL
    pub medium: Option<String>,
    /// Large image URL
    pub large: Option<String>,
    /// Original image URL
    pub original: Option<String>,
}

/// Public API user information
#[derive(Deserialize, Debug, Clone)]
pub struct PublicUser {
    /// User ID
    pub id: u64,
    /// Username
    pub name: String,
    /// User avatar URL
    pub profile_image_urls: ProfileImageUrls,
    /// Is followed
    pub is_followed: Option<bool>,
}

/// Avatar URL collection
#[derive(Deserialize, Debug, Clone)]
pub struct ProfileImageUrls {
    /// Avatar URL
    pub px_16x16: Option<String>,
    pub px_50x50: Option<String>,
    pub px_170x170: Option<String>,
}

/// Statistics information
#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    /// Bookmark count
    pub bookmarks_count: u32,
    /// Comment count
    pub comments_count: u32,
    /// View count
    pub views_count: u32,
    /// Score
    pub score: u32,
}

/// Public API search response
#[derive(Deserialize, Debug)]
pub struct PublicSearchResponse {
    /// Search result list
    pub illusts: Vec<PublicIllust>,
    /// Next page URL (if any)
    pub next_url: Option<String>,
    /// Search metadata
    pub search_meta: Option<SearchMeta>,
}

/// Search metadata
#[derive(Deserialize, Debug)]
pub struct SearchMeta {
    /// Search ID
    pub search_id: String,
    /// Search time
    pub time: u64,
}

/// Public API user detail response
#[derive(Deserialize, Debug)]
pub struct PublicUserDetail {
    /// User details
    pub user: PublicUser,
    /// User works list
    pub illusts: HashMap<String, PublicIllust>,
    /// User manga list
    pub manga: HashMap<String, PublicIllust>,
    /// User novel list
    pub novels: HashMap<String, PublicNovel>,
    /// User information
    pub profile: UserProfile,
    /// User statistics information
    pub profile_publicity: UserProfilePublicity,
    /// User workspace information
    pub workspace: Workspace,
}

/// User novel information
#[derive(Deserialize, Debug)]
pub struct PublicNovel {
    /// Novel ID
    pub id: u64,
    /// Novel title
    pub title: String,
    /// Novel tags
    pub tags: Vec<String>,
    /// Novel description
    pub caption: String,
    /// Novel cover URL
    pub image_urls: ImageUrls,
    /// Author information
    pub user: PublicUser,
    /// Creation time
    pub create_date: String,
    /// Text length
    pub text_length: u32,
    /// Total page count
    pub page_count: u32,
    /// Visible page count
    pub visible_page_count: u32,
    /// Is R18 content
    pub is_muted: bool,
}

/// User information
#[derive(Deserialize, Debug)]
pub struct UserProfile {
    /// Birthday
    pub birth: String,
    /// Birthday visibility
    pub birth_day: String,
    /// Birth year
    pub birth_year: i32,
    /// Gender
    pub gender: String,
    /// Region
    pub region: String,
    /// Address ID
    pub address_id: u32,
    /// Country code
    pub country_code: String,
    /// Job
    pub job: String,
    /// 职业ID
    pub job_id: u32,
    /// Interests
    pub interests: String,
    /// Comment
    pub comment: Option<String>,
    /// Webpage
    pub webpage: Option<String>,
    /// Twitter account
    pub twitter_account: Option<String>,
    /// Twitter URL
    pub twitter_url: Option<String>,
    /// Pawoo URL
    pub pawoo_url: Option<String>,
    /// Is Pawoo user
    pub is_pawoo_user: Option<bool>,
}

/// User information publicity
#[derive(Deserialize, Debug)]
pub struct UserProfilePublicity {
    /// Birthday visibility
    pub birth_day: String,
    /// Gender visibility
    pub gender: String,
    /// Region visibility
    pub region: String,
    /// Job visibility
    pub job: String,
    /// Interests visibility
    pub interests: String,
    /// Comment visibility
    pub comment: String,
    /// Webpage visibility
    pub webpage: String,
    /// Twitter visibility
    pub twitter: String,
    /// Pawoo visibility
    pub pawoo: String,
}

/// Workspace information
#[derive(Deserialize, Debug)]
pub struct Workspace {
    /// Desk
    pub desk: Option<String>,
    /// Chair
    pub chair: Option<String>,
    /// Operating system
    pub operating_system: Option<String>,
    /// Tool
    pub tool: Option<String>,
    /// Comment
    pub comment: Option<String>,
    /// Music
    pub music: Option<String>,
    /// Tea
    pub tea: Option<String>,
    /// PC
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

/// Public API user works response
#[derive(Deserialize, Debug)]
pub struct PublicUserIllusts {
    /// User works list
    pub illusts: Vec<PublicIllust>,
    /// Next page URL (if any)
    pub next_url: Option<String>,
}

/// Public API user bookmarks response
#[derive(Deserialize, Debug)]
pub struct PublicUserBookmarks {
    /// Bookmarked works list
    pub illusts: Vec<PublicIllust>,
    /// Next page URL (if any)
    pub next_url: Option<String>,
}

/// Search target type
#[derive(Debug, Clone)]
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

/// Sort method
#[derive(Debug, Clone)]
pub enum Sort {
    /// Date descending
    DateDesc,
    /// Date ascending
    DateAsc,
    /// Popular descending
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

/// Content type
#[derive(Debug, Clone)]
pub enum ContentType {
    /// Illustration
    Illust,
    /// Manga
    Manga,
    /// Ugoira
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

/// Filter type
#[derive(Debug, Clone)]
pub enum Filter {
    /// iOS filter
    ForIOS,
    /// No filter
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

/// Search duration
#[derive(Debug, Clone)]
pub enum Duration {
    /// Within last day
    WithinLastDay,
    /// Within last week
    WithinLastWeek,
    /// Within last month
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

/// Access restriction type
#[derive(Debug, Clone)]
pub enum Restrict {
    Public,  // Public
    Private, // Private
}

impl std::fmt::Display for Restrict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Restrict::Public => write!(f, "public"),
            Restrict::Private => write!(f, "private"),
        }
    }
}