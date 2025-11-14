# Data Models & Schemas

## Overview

PixivPy uses Pydantic models for type-safe data structures. These models define the structure of API responses, providing runtime validation, type hints, and documentation. This document details all data models used throughout the library.

## Model Organization

### Directory Structure
```
pixivpy/
├── models/
│   ├── __init__.py
│   ├── base.py          # Base model classes
│   ├── user.py          # User-related models
│   ├── illust.py        # Illustration models
│   ├── novel.py         # Novel models
│   ├── search.py        # Search result models
│   ├── bookmark.py      # Bookmark models
│   ├── series.py        # Series models
│   └── comment.py       # Comment models
```

## Base Models

### 1. BaseResponse
```python
from pydantic import BaseModel
from typing import Optional, Dict, Any

class BaseResponse(BaseModel):
    """Base class for all API responses"""
    class Meta(BaseModel):
        status: int
        error: Optional[str] = None
        message: Optional[str] = None

    meta: Meta
```

### 2. BaseImageUrls
```python
class BaseImageUrls(BaseModel):
    """Common image URL pattern"""
    square_medium: Optional[str] = None
    medium: Optional[str] = None
    large: Optional[str] = None
    original: Optional[str] = None
```

## User Models

### 1. UserInfo (Basic)
```python
class UserInfo(BaseModel):
    """Basic user information"""
    id: int
    name: str
    account: str
    profile_image_urls: BaseImageUrls
    is_followed: Optional[bool] = None
    is_muted: Optional[bool] = None

    class Config:
        allow_population_by_field_name = True
```

### 2. UserInfoDetailed
```python
class UserInfoDetailed(BaseModel):
    """Complete user profile"""
    id: int
    name: str
    account: str
    profile_image_urls: BaseImageUrls
    comment: str
    is_followed: bool
    is_muted: bool
    is_blocking: bool
    is_follower: bool
    is_friend: bool
    is_premium: bool
    background_image_urls: Optional[Dict[str, str]] = None
    twitter_account: Optional[str] = None
    twitter_url: Optional[str] = None
    webpage_url: Optional[str] = None
    gender: Optional[str] = None
    birth: Optional[str] = None
    birth_day: Optional[int] = None
    birth_month: Optional[int] = None
    birth_year: Optional[int] = None
    region: Optional[str] = None
    address_id: Optional[int] = None
    country_code: Optional[str] = None
    job: Optional[str] = None
    job_id: Optional[int] = None
    total_followers: int
    total_following: int
    total_mypixiv_users: int
    total_illusts: int
    total_manga: int
    total_novels: int
    total_bookmarked_illusts: int
    total_bookmarked_novels: int
    total_comments: int
    total_comments_received: int
    total_view: int
    total_like: int
    total_watching_users: int
    total_myrequest: int
    total_diff_importing: int
    total_diff_use_shop: int
    total_diff_use_item: int
    total_group: int
    profile_partial: Dict[str, Any] = {}
    user_ad_cooperation_id: Optional[int] = None
    user_advertiser_id: Optional[int] = None
    has_booost_promotion: bool
    following_privacy: Optional[str] = None
    request_user_status: Optional[Dict[str, Any]] = None
    can_be_followed: bool

    class Config:
        allow_population_by_field_name = True
```

## Illustration Models

### 1. ImageUrls
```python
class ImageUrls(BaseModel):
    """Illustration image URLs at different sizes"""
    square_medium: str
    medium: str
    large: str
    original: Optional[str] = None
```

### 2. SinglePage
```python
class SinglePage(BaseModel):
    """Single page illustration info"""
    width: int
    height: int
```

### 3. MultiPage
```python
class MultiPage(BaseModel):
    """Multi-page manga/novel info"""
    page_count: int
    pages: List[Dict[str, Any]]  # Contains SinglePage data
    single_page: SinglePage
```

### 4. Tag
```python
class Tag(BaseModel):
    """Illustration tag"""
    name: str
    translated_name: Optional[str] = None
    added_by_uploaded_user: Optional[bool] = None

    class Config:
        allow_population_by_field_name = True
```

### 5. IllustInfo
```python
from typing import List, Optional, Dict, Any
from datetime import datetime

class IllustInfo(BaseModel):
    """Complete illustration information"""
    id: int
    title: str
    type: str  # "illust" or "manga"
    image_urls: ImageUrls
    caption: str
    restrict: int  # 0=public, 1=private, 2=custom
    user: UserInfo
    tags: List[Tag]
    tools: List[str] = []
    create_date: datetime
    page_count: int
    width: int
    height: int
    sanity_level: int  # 0-6, content rating
    x_restrict: int  # 0=none, 1=R-18, 2=R-18G
    series: Optional[Dict[str, Any]] = None
    series_id: Optional[int] = None
    meta_single_page: Optional[Dict[str, Any]] = {}
    meta_pages: List[Dict[str, Any]] = []
    total_view: int
    total_bookmarks: int
    total_comments: int
    is_bookmarked: bool
    is_muted: bool
    visible: bool
    is_mypixiv: bool
    is_x_restricted: bool
    illust_ai_type: int  # 0=human, 1=AI, 2=unknown
    illust_book_style: int
    width_ai: int
    height_ai: int
    level: int  # Deprecated
    like_count: bool  # Deprecated
    love_count: int  # Deprecated
    comment_count: int  # Deprecated
    reform_level: int  # Deprecated
    newest_illust_ids: List[int] = []  # Deprecated
    favorite_ids: List[int] = []  # Deprecated
    bookmark_illust_ids: List[int] = []  # Deprecated
    is_masked: bool
    penalty_level: int
    penalty: Optional[Dict[str, Any]] = {}
    zone: List[str] = []
    poll_data: Optional[Dict[str, Any]] = None

    class Config:
        allow_population_by_field_name = True
        json_encoders = {
            datetime: lambda v: v.isoformat()
        }
```

## Novel Models

### 1. NovelInfo
```python
class NovelInfo(BaseModel):
    """Complete novel information"""
    id: int
    title: str
    text: str  # Only for novel_text endpoint
    text_length: int
    image_urls: Optional[Dict[str, str]] = {}
    user: UserInfo
    tags: List[Tag]
    series: Optional[Dict[str, Any]] = None
    series_id: Optional[int] = None
    create_date: datetime
    update_date: datetime
    restrict: int
    x_restrict: int
    is_original: bool
    is_bookmarked: bool
    is_muted: bool
    total_view: int
    total_bookmarks: int
    total_comments: int
    text_embedding: Optional[Dict[str, Any]] = None
    novel_ai_type: int
    visible: bool
    level: int  # Deprecated
    like_count: int  # Deprecated
    comment_count: int  # Deprecated
    bookmark_count: int  # Deprecated
    favorite_ids: List[int] = []  # Deprecated
    bookmark_illust_ids: List[int] = []  # Deprecated
    is_masked: bool
    penalty_level: int
    penalty: Optional[Dict[str, Any]] = {}

    class Config:
        allow_population_by_field_name = True
```

### 2. NovelSeriesInfo
```python
class NovelSeriesInfo(BaseModel):
    """Novel series information"""
    id: int
    title: str
    caption: str
    create_date: datetime
    update_date: datetime
    total_novels: int
    display_order: int
    is_concluded: bool
    series_work_display_order: int
    is_last_novel_new: bool
    is_first_novel_new: bool
    is_visible: bool
    is_restricted: bool
    x_restrict: int
    latest_novel: Optional[Dict[str, Any]] = None
    user: UserInfo

    class Config:
        allow_population_by_field_name = True
```

## Search Result Models

### 1. SearchIllustrations
```python
class SearchIllustrations(BaseModel):
    """Search results for illustrations"""
    illusts: List[IllustInfo] = []
    next_url: Optional[str] = None
    search_span_limit: Optional[int] = None

    class Config:
        allow_population_by_field_name = True
```

### 2. SearchNovel
```python
class SearchNovel(BaseModel):
    """Search results for novels"""
    novels: List[NovelInfo] = []
    next_url: Optional[str] = None
    search_span_limit: Optional[int] = None

    class Config:
        allow_population_by_field_name = True
```

### 3. SearchUser
```python
class SearchUser(BaseModel):
    """Search results for users"""
    user_previews: List[Dict[str, Any]] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

## Bookmark Models

### 1. UserBookmarksIllustrations
```python
class UserBookmarksIllustrations(BaseModel):
    """User's bookmarked illustrations"""
    bookmark_illusts: List[Dict[str, Any]] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

### 2. UserBookmarksNovel
```python
class UserBookmarksNovel(BaseModel):
    """User's bookmarked novels"""
    bookmark_novels: List[Dict[str, Any]] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

## User Content Models

### 1. UserIllustrations
```python
class UserIllustrations(BaseModel):
    """User's illustration collection"""
    illusts: List[IllustInfo] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

### 2. UserNovels
```python
class UserNovels(BaseModel):
    """User's novel collection"""
    novels: List[NovelInfo] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

### 3. UserFollowing
```python
class UserFollowing(BaseModel):
    """User's following list"""
    user_previews: List[Dict[str, Any]] = []
    next_url: Optional[str] = None

    class Config:
        allow_population_by_field_name = True
```

## Comment Models

### 1. Comment
```python
class Comment(BaseModel):
    """Individual comment"""
    id: int
    comment: str
    date: datetime
    user: UserInfo
    parent_comment: Optional[Dict[str, Any]] = None
    has_replies: bool
    stamp: Optional[Dict[str, Any]] = None
    reply_to_user: Optional[UserInfo] = None
    reply_comments: List[Dict[str, Any]] = []

    class Config:
        allow_population_by_field_name = True
```

### 2. CommentsInfo
```python
class CommentsInfo(BaseModel):
    """Comments collection with pagination"""
    comments: List[Comment] = []
    next_url: Optional[str] = None
    total_comments: Optional[int] = None

    class Config:
        allow_population_by_field_name = True
```

## Ranking Models

### 1. RankingInfo
```python
class RankingInfo(BaseModel):
    """Ranking information"""
    illusts: List[IllustInfo] = []
    next_url: Optional[str] = None
    last_seen: Optional[datetime] = None

    class Config:
        allow_population_by_field_name = True
```

## Trending Models

### 1. TrendingTag
```python
class TrendingTag(BaseModel):
    """Trending tag information"""
    tag: str
    translated_name: Optional[str] = None
    illust: Optional[Dict[str, Any]] = None

    class Config:
        allow_population_by_field_name = True
```

### 2. TrendingTagsInfo
```python
class TrendingTagsInfo(BaseModel):
    """Trending tags collection"""
    trend_tags: List[TrendingTag] = []

    class Config:
        allow_population_by_field_name = True
```

## Model Features

### 1. Pydantic Version Support
The library supports both Pydantic v1 and v2:
```python
try:
    from pydantic import BaseModel
    PYDANTIC_V2 = hasattr(BaseModel, 'model_rebuild')
except ImportError:
    PYDANTIC_V2 = False
```

### 2. Field Aliases
Many models use field aliases to convert between snake_case (Python) and camelCase (API):
```python
class IllustInfo(BaseModel):
    create_date: datetime = Field(..., alias="createDate")
    user_id: int = Field(..., alias="userId")

    class Config:
        allow_population_by_field_name = True
```

### 3. Empty Object Handling
Special handling for empty API responses:
```python
class EmptyModel(BaseModel):
    """Handle empty JSON objects"""
    class Config:
        extra = "forbid"
```

## Serialization/Deserialization Patterns

### 1. Response Parsing
```python
def parse_response(model_class, response_json):
    """Parse API response into model"""
    try:
        return model_class(**response_json)
    except ValidationError as e:
        # Handle validation errors
        raise PixivError(f"Invalid response format: {e}")
```

### 2. Optional Fields
Many fields are optional due to API variations:
```python
class IllustInfo(BaseModel):
    series: Optional[Dict[str, Any]] = None  # Not always present
    series_id: Optional[int] = None  # Might be null
```

### 3. Nested Models
Complex nested structures for rich data:
```python
class UserWithWorks(BaseModel):
    user: UserInfo
    illusts: List[IllustInfo]
    novels: List[NovelInfo]
    total_illusts: int
    total_novels: int
```

## Rust Implementation Strategy

### 1. Serde Models
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
    pub account: String,
    pub profile_image_urls: ImageUrls,
    #[serde(default)]
    pub is_followed: Option<bool>,
    #[serde(default)]
    pub is_muted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrls {
    #[serde(rename = "square_medium")]
    pub squareMedium: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
}
```

### 2. Enums for Fixed Values
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Illust,
    Manga,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictType {
    Public,
    Private,
}
```

### 3. Optional Fields Handling
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IllustInfo {
    pub id: u64,
    pub title: String,
    pub r#type: ContentType,  // Raw "type" keyword
    pub image_urls: ImageUrls,
    pub caption: String,
    pub user: UserInfo,
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub series: Option<SeriesInfo>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(with = "custom_date_format")]
    pub create_date: DateTime<Utc>,
    #[serde(default)]
    pub x_restrict: u8,  // Content rating
    #[serde(default)]
    pub illust_ai_type: u8,  // AI content flag
}
```

### 4. Custom Date Handling
```rust
mod custom_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map(|nd| DateTime::from_utc(nd, Utc))
            .map_err(serde::de::Error::custom)
    }
}
```

### 5. Pagination Models
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    #[serde(flatten)]
    pub data: T,
    #[serde(rename = "nextUrl")]
    pub next_url: Option<String>,
}

impl<T> PaginatedResponse<T> {
    pub fn has_more(&self) -> bool {
        self.next_url.is_some()
    }
}
```

### 6. Error Handling for Validation
```rust
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Invalid field {field}: {message}")]
    InvalidField { field: String, message: String },

    #[error("Missing required field {field}")]
    MissingField { field: String },

    #[error("Type conversion error: {0}")]
    ConversionError(String),
}

// Result type alias
pub type ModelResult<T> = Result<T, ModelError>;
```

### 7. Traits for Common Operations
```rust
pub trait ResponseModel {
    fn from_json(json: &str) -> Result<Self, ModelError>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        serde_json::from_str(json).map_err(|e| {
            ModelError::ConversionError(e.to_string())
        })
    }
}

pub trait Paginated {
    fn next_url(&self) -> Option<&str>;
    fn has_next_page(&self) -> bool;
}

impl<T> Paginated for PaginatedResponse<T> {
    fn next_url(&self) -> Option<&str> {
        self.next_url.as_deref()
    }

    fn has_next_page(&self) -> bool {
        self.next_url.is_some()
    }
}
```

## Best Practices

1. **Use enums** for fixed string values (content types, restrict modes)
2. **Handle null vs empty** properly with Option types
3. **Document deprecated fields** but keep them for compatibility
4. **Use proper date/time handling** with timezone awareness
5. **Implement Debug** for all models for easier debugging
6. **Consider versioning** models for API changes
7. **Use serde attributes** for field name conversions
8. **Validate critical fields** after deserialization
9. **Provide defaults** for optional API fields
10. **Test with real API responses** to ensure accuracy