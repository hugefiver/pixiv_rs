# API Endpoints Reference

## Overview

PixivPy implements over 50 API endpoints across four main categories: Illustrations, Users, Novels, and Search/Discovery. Each endpoint returns structured data using Pydantic models for type safety.

## Base API Configuration

### Base URL
- Primary: `https://app-api.pixiv.net`
- All endpoints require authentication via Bearer token

### Common Headers
```http
Authorization: Bearer {access_token}
User-Agent: PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)
Accept-Language: en-us (optional, for tag translations)
```

### Common Parameters
- `filter`: `"for_ios"` (default) or `""` (no filter)
- `restrict`: `"public"` or `"private"` (for bookmarks/following)

## 1. Illustration APIs

### 1.1 Get Illustration Details
```python
api.illust_detail(illust_id: int) -> ParsedJson
```
- **Endpoint**: `GET /v1/illust/detail`
- **Parameters**:
  - `illust_id`: Illustration ID
- **Returns**: Complete illustration metadata as parsed JSON (dict)

### 1.2 Illustration Rankings
```python
api.illust_ranking(
    mode: str = "day",
    filter: str = "for_ios"
) -> RankingInfo
```
- **Endpoint**: `GET /v1/illust/ranking`
- **Modes**:
  - `day`: Daily ranking
  - `week`: Weekly ranking
  - `month`: Monthly ranking
  - `day_male`: Daily (male)
  - `day_female`: Daily (female)
  - `week_original`: Weekly (original)
  - `week_rookie`: Weekly (rookie)
  - `day_r18`: Daily (R-18)
  - `day_male_r18`, `day_female_r18`, `week_r18`, etc.
- **Returns**: Ranked illustrations with pagination

### 1.3 Recommended Illustrations
```python
api.illust_recommended(
    content_type: str = "illust",
    include_ranking_label: bool = True,
    filter: str = "for_ios",
    max_bookmark_id_for_recommend: int = None,
    min_bookmark_id_for_recent_illust: int = None,
    offset: int = None,
    include_ranking_illusts: bool = None,
    bookmark_illust_ids: Union[str, List[Union[int, str]]] = None,
    include_privacy_policy: Union[str, List[Union[int, str]]] = None,
    viewed: Union[str, List[str]] = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/illust/recommended`
- **Features**: Personalized recommendations based on user preferences
- **Returns**: Recommended illustrations with variety as parsed JSON (dict)

### 1.4 Related Illustrations
```python
api.illust_related(
    illust_id: int,
    filter: str = "for_ios",
    seed_illust_ids: Union[str, List[str]] = None,
    offset: int = None,
    viewed: Union[str, List[str]] = None
) -> ParsedJson
```
- **Endpoint**: `GET /v2/illust/related`
- **Features**: Illustrations similar to the given one
- **Returns**: Related works as parsed JSON (dict)

### 1.5 Following Users' New Works
```python
api.illust_follow(
    restrict: str = "public",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v2/illust/follow`
- **Parameters**:
  - `restrict`: `public` (from public follows) or `private` (from private follows)
- **Returns**: New illustrations from followed users as parsed JSON (dict)

### 1.6 Latest Illustrations
```python
api.illust_new(
    content_type: str = "illust",
    filter: str = "for_ios",
    max_illust_id: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/illust/new`
- **Parameters**:
  - `content_type`: `illust` or `manga`
  - `max_illust_id`: Get works older than this ID
- **Returns**: Latest public illustrations as parsed JSON (dict)

### 1.7 Illustration Comments
```python
api.illust_comments(
    illust_id: int,
    offset: int = None,
    include_total_comments: bool = True
) -> ParsedJson
```
- **Endpoint**: `GET /v1/illust/comments`
- **Features**: Pagination support for comments
- **Returns**: Comment tree structure as parsed JSON (dict)

### 1.8 Illustration Bookmarks
```python
api.illust_bookmark_detail(illust_id: int) -> ParsedJson
api.illust_bookmark_add(
    illust_id: int,
    restrict: str = "public",
    tags: List[str] = None
) -> ParsedJson
api.illust_bookmark_delete(illust_id: int) -> ParsedJson
```
- **Endpoints**:
  - `GET /v2/illust/bookmark/detail`
  - `POST /v2/illust/bookmark/add`
  - `POST /v1/illust/bookmark/delete`
- **Returns**: Bookmark status/result as parsed JSON (dict)

### 1.9 Ugoira Metadata
```python
api.ugoira_metadata(illust_id: int) -> ParsedJson
```
- **Endpoint**: `GET /v1/ugoira/metadata`
- **Returns**: Metadata for animated illustrations (ugoira) as parsed JSON (dict)

## 2. User APIs

### 2.1 User Profile
```python
api.user_detail(
    user_id: int,
    filter: str = "for_ios"
) -> UserInfoDetailed
```
- **Endpoint**: `GET /v1/user/detail`
- **Returns**: Complete user profile including stats (Model)

### 2.2 User's Illustrations
```python
api.user_illusts(
    user_id: int,
    filter: str = "for_ios",
    offset: int = None,
    type: str = None
) -> UserIllustrations
```
- **Endpoint**: `GET /v1/user/illusts`
- **Parameters**:
  - `type`: `illust`, `manga`, or `None` (both)
- **Returns**: User's illustration collection (Model)

### 2.3 User's Novels
```python
api.user_novels(
    user_id: int,
    filter: str = "for_ios",
    offset: int = None
) -> UserNovels
```
- **Endpoint**: `GET /v1/user/novels`
- **Returns**: User's novel collection (Model)

### 2.4 User's Bookmarked Illustrations
```python
api.user_bookmarks_illust(
    user_id: int,
    restrict: str = "public",
    filter: str = "for_ios",
    max_bookmark_id: int = None,
    tag: str = None
) -> UserBookmarksIllustrations
```
- **Endpoint**: `GET /v1/user/bookmarks/illust`
- **Parameters**:
  - `restrict`: `public` or `private` bookmarks
  - `tag`: Filter by bookmark tag
- **Returns**: User's bookmarked illustrations (Model)

### 2.5 User's Bookmarked Novels
```python
api.user_bookmarks_novel(
    user_id: int,
    restrict: str = "public",
    filter: str = "for_ios",
    max_bookmark_id: int = None
) -> UserBookmarksNovel
```
- **Endpoint**: `GET /v1/user/bookmarks/novel`
- **Returns**: User's bookmarked novels (Model)

### 2.6 User's Following List
```python
api.user_following(
    user_id: int,
    restrict: str = "public",
    offset: int = None
) -> UserFollowing
```
- **Endpoint**: `GET /v1/user/following`
- **Returns**: Users that the target user follows (Model)

### 2.7 User's Followers
```python
api.user_follower(
    user_id: int,
    filter: str = "for_ios",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/user/follower`
- **Returns**: Users that follow the target user as parsed JSON (dict)

### 2.8 Follow/Unfollow User
```python
api.user_follow_add(user_id: int, restrict: str = "public") -> ParsedJson
api.user_follow_delete(user_id: int) -> ParsedJson
```
- **Endpoints**:
  - `POST /v1/user/follow/add`
  - `POST /v1/user/follow/delete`
- **Parameters**:
  - `restrict`: `public` or `private` (for follow)
- **Returns**: Follow status as parsed JSON (dict)

### 2.9 Related Users
```python
api.user_related(
    seed_user_id: int,
    filter: str = "for_ios"
) -> ParsedJson
```
- **Endpoint**: `GET /v1/user/related`
- **Returns**: Users similar to the seed user as parsed JSON (dict)

### 2.10 My Pixiv Users
```python
api.user_mypixiv(
    user_id: int,
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/user/mypixiv`
- **Returns**: "My Pixiv" users list as parsed JSON (dict)

### 2.11 User Blacklist
```python
api.user_list(
    user_id: int,
    filter: str = "for_ios",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v2/user/list`
- **Returns**: Blacklisted users as parsed JSON (dict)

### 2.12 User Bookmark Tags
```python
api.user_bookmark_tags_illust(
    user_id: int,
    restrict: str = "public",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/user/bookmark-tags/illust`
- **Returns**: Tags used in user's bookmarks as parsed JSON (dict)

### 2.13 User AI Settings
```python
api.user_edit_ai_show_settings(setting: str) -> ParsedJson
```
- **Endpoint**: `POST /v1/user/ai-show-settings/edit`
- **Parameters**:
  - `setting`: `"true"` or `"false"`
- **Returns**: Result of updating AI display settings as parsed JSON (dict)

## 3. Novel APIs

### 3.1 Novel Details
```python
api.novel_detail(novel_id: int) -> NovelInfo
```
- **Endpoint**: `GET /v2/novel/detail`
- **Returns**: Complete novel metadata (Model)

### 3.2 Novel Content (Deprecated)
```python
api.novel_text(novel_id: int) -> WebviewNovel
```
- **Endpoint**: `GET /v1/novel/text`
- **Status**: Deprecated, use `webview_novel` instead

### 3.3 Novel Content (Current)
```python
api.webview_novel(novel_id: int) -> WebviewNovel
```
- **Endpoint**: `GET /webview/v2/novel`
- **Returns**: Formatted novel content (Model)

### 3.4 Novel Series
```python
api.novel_series(series_id: int) -> ParsedJson
```
- **Endpoint**: `GET /v2/novel/series`
- **Returns**: Novel series information as parsed JSON (dict)

### 3.5 Novel Comments
```python
api.novel_comments(
    novel_id: int,
    offset: int = None,
    include_total_comments: bool = True
) -> NovelComments
```
- **Endpoint**: `GET /v3/novel/comments`
- **Returns**: Comments on the novel (Model)

### 3.6 Following Users' New Novels
```python
api.novel_follow(
    restrict: str = "public",
    filter: str = "for_ios",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/novel/follow`
- **Returns**: New novels from followed users as parsed JSON (dict)

### 3.7 Latest Novels
```python
api.novel_new(
    filter: str = "for_ios",
    max_novel_id: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/novel/new`
- **Returns**: Latest public novels as parsed JSON (dict)

### 3.8 Recommended Novels
```python
api.novel_recommended(
    include_ranking_label: bool = True,
    filter: str = "for_ios",
    offset: int = None,
    include_ranking_novels: bool = None,
    already_recommended: List[str] = None,
    max_bookmark_id_for_recommend: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/novel/recommended`
- **Returns**: Personalized novel recommendations as parsed JSON (dict)

## 4. Search and Discovery APIs

### 4.1 Search Illustrations
```python
api.search_illust(
    word: str,
    search_target: str = "partial_match_for_tags",
    sort: str = "date_desc",
    duration: str = None,
    start_date: str = None,
    end_date: str = None,
    filter: str = "for_ios",
    offset: int = None,
    search_ai_type: int = None
) -> SearchIllustrations
```
- **Endpoint**: `GET /v1/search/illust`
- **Returns**: Search results (Model)

### 4.2 Search Novels
```python
api.search_novel(
    word: str,
    search_target: str = "partial_match_for_tags",
    sort: str = "date_desc",
    merge_plain_keyword_results: bool = True,
    include_translated_tag_results: bool = True,
    duration: str = None,
    start_date: str = None,
    end_date: str = None,
    filter: str = "for_ios",
    search_ai_type: int = None,
    offset: int = None
) -> SearchNovel
```
- **Endpoint**: `GET /v1/search/novel`
- **Returns**: Search results (Model)

### 4.3 Search Users
```python
api.search_user(
    word: str,
    filter: str = "for_ios",
    offset: int = None
) -> ParsedJson
```
- **Endpoint**: `GET /v1/search/user`
- **Returns**: Users matching the search term as parsed JSON (dict)

### 4.4 Trending Tags for Illustrations
```python
api.trending_tags_illust(filter: str = "for_ios") -> ParsedJson
```
- **Endpoint**: `GET /v1/trending-tags/illust`
- **Returns**: Currently trending illustration tags as parsed JSON (dict)


### 4.5 Trending Tags for Novels
```python
api.trending_tags_novel(filter: str = "for_ios") -> NovelTrendingTagsInfo
```
- **Endpoint**: `GET /v1/trending-tags/novel`
- **Returns**: Currently trending novel tags

## 5. Utility APIs

### 5.1 Parse Next URL
```python
api.parse_qs(next_url: str) -> Dict[str, Any]
```
- **Utility**: Extract query parameters from `next_url` for pagination
- **Usage**:
```python
result = api.search_illust("watercolor")
while result.next_url:
    next_qs = api.parse_qs(result.next_url)
    result = api.search_illust(**next_qs)
```

### 5.2 Download Illustration
```python
api.download(
    url: str,
    path: str = None,
    prefix: str = None,
    ext: str = None,
    replace: bool = False
) -> str
```
- **Features**:
  - Automatic referer header for Pixiv images
  - Progress tracking
  - File extension detection
  - Resume support
- **Returns**: Downloaded file path

### 5.3 Showcase Article
```python
api.showcase_article(showcase_id: int) -> ShowcaseInfo
```
- **Endpoint**: `GET https://www.pixiv.net/ajax/showcase/article`
- **Note**: This is a Web API call (no authentication required, mocks Chrome User-Agent)
- **Returns**: Article content and metadata

## 6. Response Models

### 6.1 Common Response Structure
```python
class BaseResponse:
    # All API responses follow this pattern
    class Meta:
        status: int
        error: Optional[str]
    class Data:
        # Actual data varies by endpoint
```

### 6.2 Pagination Information
```python
class PaginatedResponse:
    next_url: Optional[str]  # URL for next page
    # Data items
```

## 7. Request Patterns

### 7.1 GET Requests
Most endpoints use GET with query parameters:
```python
api.illust_detail(59580629)
# -> GET /v1/illust/detail?illust_id=59580629
```

### 7.2 POST Requests
State-changing operations use POST:
```python
api.user_follow_add(12345)
# -> POST /v1/user/follow/add with form data
```

### 7.3 Content Filtering
All data endpoints support filtering:
```python
api.search_illust("tag", filter="for_ios")  # Default
api.search_illust("tag", filter="")        # No filtering
```

## 8. Rate Limits and Best Practices

### 8.1 Rate Limit Headers
```http
X-RateLimit-Limit: 120
X-RateLimit-Remaining: 118
X-RateLimit-Reset: 1640995200
```

### 8.2 Best Practices
1. **Implement delays** between requests (recommended: 1 second)
2. **Use pagination** efficiently with `next_url`
3. **Cache responses** for repeated requests
4. **Handle rate limits** gracefully
5. **Prefer specific endpoints** over generic search when possible

## 9. Example API Calls

### 9.1 Complete Workflow
```python
# Initialize and authenticate
api = AppPixivAPI()
api.auth(refresh_token="TOKEN")

# Search for illustrations
result = api.search_illust("landscape", sort="popular_desc")
for illust in result.illusts:
    print(f"Title: {illust.title}")
    print(f"Author: {illust.user.name}")

    # Get detailed info
    detail = api.illust_detail(illust.id)
    print(f"Views: {detail.illust.total_view}")

# Paginate through results
while result.next_url:
    next_qs = api.parse_qs(result.next_url)
    result = api.search_illust(**next_qs)
    # Process next page...
```

### 9.2 Advanced Search with Filters
```python
# Search recent illustrations, exclude AI content
result = api.search_illust(
    word="original character",
    search_target="partial_match_for_tags",
    sort="date_desc",
    duration="within_last_week",
    search_ai_type=1  # Exclude AI
)

# Filter by popularity
result = api.search_illust(
    word="fantasy art",
    sort="popular_desc",
    filter="for_ios"
)
```

### 9.3 User Data Collection
```python
# Get user profile and all their works
user_detail = api.user_detail(user_id=12345)
print(f"User: {user_detail.user.name}")

# Get all illustrations with pagination
illusts_result = api.user_illusts(user_id=12345)
all_illusts = list(illusts_result.illusts)

while illusts_result.next_url:
    next_qs = api.parse_qs(illusts_result.next_url)
    illusts_result = api.user_illusts(user_id=12345, **next_qs)
    all_illusts.extend(illusts_result.illusts)
```

## 10. Error Responses

### 10.1 Common Error Codes
- `200`: Success
- `400`: Bad Request (invalid parameters)
- `401`: Unauthorized (invalid/expired token)
- `403`: Forbidden (no access)
- `404`: Not Found
- `429`: Rate Limited
- `500`: Internal Server Error

### 10.2 Error Response Format
```json
{
    "has_error": true,
    "errors": {
        "system": {
            "message": "Error description"
        }
    }
}
```

## Rust Implementation Considerations

### 1. Endpoint Organization
```rust
pub mod endpoints {
    pub mod illust;
    pub mod user;
    pub mod novel;
    pub mod search;
}
```

### 2. Common Traits
```rust
pub trait PaginatedResponse {
    fn next_url(&self) -> Option<&str>;
    fn has_more(&self) -> bool;
}

pub trait Endpoint {
    type Response;
    type Parameters;

    fn endpoint() -> &'static str;
    fn method() -> Method;
}
```

### 3. Type-safe Parameters
```rust
#[derive(Debug, Clone, Copy)]
pub enum SearchTarget {
    PartialMatchForTags,
    ExactMatchForTags,
    TitleAndCaption,
    Keyword,
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    DateDesc,
    DateAsc,
    PopularDesc,
}
```

### 4. Async Implementation
```rust
impl PixivClient {
    pub async fn search_illust(
        &self,
        params: SearchIllustParams,
    ) -> Result<SearchIllustrationsResponse, PixivError> {
        let url = format!("{}/v1/search/illust", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
```