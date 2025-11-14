# Usage Patterns & Examples

## Overview

This document outlines common usage patterns and examples when using PixivPy. These patterns demonstrate best practices for typical operations including authentication, data retrieval, pagination, file downloads, and error handling.

## Basic Setup and Authentication

### 1. Standard Initialization

```python
from pixivpy3 import AppPixivAPI
import os

# Initialize client
api = AppPixivAPI()

# Authenticate with refresh token
refresh_token = os.getenv('PIXIV_REFRESH_TOKEN')
api.auth(refresh_token=refresh_token)

# You're now ready to make API calls
```

### 2. With Proxy Configuration

```python
proxies = {
    'http': 'http://proxy.example.com:8080',
    'https': 'http://proxy.example.com:8080'
}

api = AppPixivAPI()
api.set_proxies(proxies)
api.auth(refresh_token=refresh_token)
```

### 3. Network Bypass Initialization

```python
from pixivpy3 import ByPassSniApi

# For networks that block Pixiv
api = ByPassSniApi()
api.require_appapi_hosts()  # Resolve real IPs
api.auth(refresh_token=refresh_token)
```

## Common Workflows

### 1. Search and Download Workflow

```python
def search_and_download(keyword, max_results=50, download_dir="downloads"):
    """Search for illustrations and download them"""

    import os
    from pathlib import Path

    # Create download directory
    Path(download_dir).mkdir(exist_ok=True)

    # Search
    result = api.search_illust(
        word=keyword,
        sort="popular_desc",
        search_target="partial_match_for_tags"
    )

    downloaded = 0
    for illust in result.illusts:
        if downloaded >= max_results:
            break

        # Skip manga with multiple pages for simplicity
        if illust.page_count > 1:
            continue

        # Get image URL
        image_url = illust.image_urls.large

        # Create filename
        filename = f"{illust.id}_{illust.title.replace('/', '_')}.jpg"
        filepath = os.path.join(download_dir, filename)

        # Download
        try:
            api.download(image_url, path=filepath)
            downloaded += 1
            print(f"Downloaded: {illust.title} by {illust.user.name}")
        except Exception as e:
            print(f"Failed to download {illust.id}: {e}")

    print(f"Successfully downloaded {downloaded} images")
```

### 2. User Data Collection Workflow

```python
def collect_user_illustrations(user_id):
    """Collect all illustrations from a user"""

    all_illusts = []
    offset = 0

    while True:
        # Get user's illustrations
        result = api.user_illusts(
            user_id=user_id,
            offset=offset
        )

        if not result.illusts:
            break

        all_illusts.extend(result.illusts)
        offset += len(result.illusts)

        print(f"Collected {len(all_illusts)} illustrations...")

    # Analyze data
    total_views = sum(illust.total_view for illust in all_illusts)
    total_bookmarks = sum(illust.total_bookmarks for illust in all_illusts)
    avg_views = total_views / len(all_illusts) if all_illusts else 0

    print(f"\nUser Statistics:")
    print(f"Total illustrations: {len(all_illusts)}")
    print(f"Total views: {total_views:,}")
    print(f"Total bookmarks: {total_bookmarks:,}")
    print(f"Average views per illust: {avg_views:.2f}")

    return all_illusts
```

### 3. Ranking Monitoring Workflow

```python
def monitor_rankings(mode="day", output_file="rankings.json"):
    """Monitor daily rankings and track changes"""

    import json
    import time
    from datetime import datetime

    rankings_data = []

    while True:
        try:
            # Get current rankings
            result = api.illust_ranking(mode=mode)

            # Process rankings
            ranking_entry = {
                "timestamp": datetime.now().isoformat(),
                "rankings": []
            }

            for idx, illust in enumerate(result.illusts[:20]):  # Top 20
                ranking_info = {
                    "rank": idx + 1,
                    "id": illust.id,
                    "title": illust.title,
                    "user": illust.user.name,
                    "views": illust.total_view,
                    "bookmarks": illust.total_bookmarks,
                    "tags": [tag.name for tag in illust.tags]
                }
                ranking_entry["rankings"].append(ranking_info)

            rankings_data.append(ranking_entry)

            # Save to file
            with open(output_file, 'w') as f:
                json.dump(rankings_data, f, indent=2)

            print(f"Updated rankings for {datetime.now().strftime('%Y-%m-%d %H:%M')}")

            # Wait for next update (Pixiv updates hourly)
            time.sleep(3600)  # 1 hour

        except Exception as e:
            print(f"Error monitoring rankings: {e}")
            time.sleep(300)  # Wait 5 minutes before retry
```

## Pagination Patterns

### 1. Standard Pagination with parse_qs

```python
def paginate_search(keyword, max_pages=5):
    """Paginate through search results using parse_qs"""

    all_results = []
    next_qs = None

    for page in range(max_pages):
        if next_qs:
            result = api.search_illust(**next_qs)
        else:
            result = api.search_illust(word=keyword)

        if not result.illusts:
            break

        all_results.extend(result.illusts)
        print(f"Page {page + 1}: Found {len(result.illusts)} items")

        # Check for next page
        if result.next_url:
            next_qs = api.parse_qs(result.next_url)
        else:
            break

    print(f"Total items collected: {len(all_results)}")
    return all_results
```

### 2. Manual Pagination with Offset

```python
def paginate_by_offset(user_id, limit=100):
    """Manual pagination using offset parameter"""

    all_items = []
    offset = 0

    while True:
        result = api.user_bookmarks_illust(
            user_id=user_id,
            offset=offset
        )

        if not result.bookmark_illusts:
            break

        all_items.extend(result.bookmark_illusts)
        offset += len(result.bookmark_illusts)

        print(f"Collected {len(all_items)} bookmarks...")

        # Stop if we've reached the limit
        if len(all_items) >= limit:
            break

        # Stop if no more items
        if not result.next_url:
            break

    return all_items[:limit]
```

### 3. Efficient Pagination with Generators

```python
def paginate_generator(endpoint, **params):
    """Generator for efficient pagination"""

    while True:
        result = endpoint(**params)

        for item in getattr(result, _get_result_key(result), []):
            yield item

        if result.next_url:
            params = api.parse_qs(result.next_url)
        else:
            break

def _get_result_key(result):
    """Get the key containing the results based on endpoint"""
    for attr in ['illusts', 'bookmarks', 'user_previews', 'novels']:
        if hasattr(result, attr):
            return attr
    return 'items'

# Usage
for illust in paginate_generator(api.search_illust, word="landscape"):
    print(f"ID: {illust.id}, Title: {illust.title}")
```

## Advanced Search Patterns

### 1. Multi-criteria Search

```python
def advanced_search(criteria):
    """Advanced search with multiple filters"""

    results = []

    # Search by primary keyword
    result = api.search_illust(
        word=criteria.get('keyword', ''),
        search_target="partial_match_for_tags",
        sort=criteria.get('sort', 'date_desc'),
        duration=criteria.get('duration', None)
    )

    # Filter results
    for illust in result.illusts:
        # Check view count
        if criteria.get('min_views') and illust.total_view < criteria['min_views']:
            continue

        # Check bookmark count
        if criteria.get('min_bookmarks') and illust.total_bookmarks < criteria['min_bookmarks']:
            continue

        # Check for specific tags
        if criteria.get('required_tags'):
            illust_tags = set(tag.name.lower() for tag in illust.tags)
            required = set(t.lower() for t in criteria['required_tags'])
            if not required.issubset(illust_tags):
                continue

        # Exclude certain tags
        if criteria.get('exclude_tags'):
            illust_tags = set(tag.name.lower() for tag in illust.tags)
            exclude = set(t.lower() for t in criteria['exclude_tags'])
            if illust_tags.intersection(exclude):
                continue

        # Check if AI-generated
        if criteria.get('exclude_ai') and illust.illust_ai_type == 1:
            continue

        results.append(illust)

    return results

# Example usage
criteria = {
    'keyword': 'character design',
    'min_views': 1000,
    'min_bookmarks': 100,
    'required_tags': ['original', 'character'],
    'exclude_tags': ['nsfw'],
    'sort': 'popular_desc',
    'exclude_ai': True
}
```

### 2. Trending Tags Analysis

```python
def analyze_trending_tags():
    """Analyze trending tags and statistics"""

    # Get trending tags
    result = api.trending_tags_illust()

    tag_stats = {}
    for trending_tag in result.trend_tags[:50]:  # Top 50
        tag = trending_tag.tag
        translated_name = trending_tag.translated_name or tag

        if trending_tag.illust:
            sample = trending_tag.illust
            stats = {
                'name': tag,
                'translated': translated_name,
                'sample_id': sample.id,
                'sample_views': getattr(sample, 'total_view', 0),
                'sample_bookmarks': getattr(sample, 'total_bookmarks', 0),
            }

            # Get more data for this tag
            search_result = api.search_illust(
                word=tag,
                sort="popular_desc",
                duration="within_last_week"
            )

            if search_result.illusts:
                # Calculate averages
                avg_views = sum(i.total_view for i in search_result.illusts) / len(search_result.illusts)
                avg_bookmarks = sum(i.total_bookmarks for i in search_result.illusts) / len(search_result.illusts)

                stats.update({
                    'weekly_posts': len(search_result.illusts),
                    'avg_views': avg_views,
                    'avg_bookmarks': avg_bookmarks,
                    'top_illust_id': search_result.illusts[0].id
                })

        tag_stats[tag] = stats

    # Sort by popularity
    sorted_tags = sorted(
        tag_stats.items(),
        key=lambda x: x[1].get('avg_bookmarks', 0),
        reverse=True
    )

    # Print results
    print("Top Trending Tags (by average bookmarks):")
    print("=" * 50)
    for tag, stats in sorted_tags[:10]:
        print(f"{stats['translated']} ({stats['name']})")
        print(f"  Posts this week: {stats.get('weekly_posts', 'N/A')}")
        print(f"  Avg bookmarks: {stats.get('avg_bookmarks', 0):.1f}")
        print(f"  Avg views: {stats.get('avg_views', 0):.1f}")
        print()

    return tag_stats
```

## Batch Operations

### 1. Batch Download with Progress

```python
from tqdm import tqdm
import concurrent.futures
import time

def batch_download_illusts(illusts, download_dir="downloads", max_workers=5):
    """Download multiple illustrations concurrently with progress bar"""

    Path(download_dir).mkdir(exist_ok=True)

    def download_single(illust):
        """Download a single illustration"""
        try:
            # Handle multi-page works
            if illust.page_count > 1:
                # Download all pages
                for page_num in range(illust.page_count):
                    if page_num < len(illust.meta_pages):
                        page_url = illust.meta_pages[page_num]['image_urls']['large']
                        filename = f"{illust.id}_p{page_num}.jpg"
                        filepath = os.path.join(download_dir, filename)
                        api.download(page_url, path=filepath)
                return f"Downloaded {illust.page_count} pages for ID {illust.id}"
            else:
                # Single page
                image_url = illust.image_urls.large
                filename = f"{illust.id}.jpg"
                filepath = os.path.join(download_dir, filename)
                api.download(image_url, path=filepath)
                return f"Downloaded ID {illust.id}"

        except Exception as e:
            return f"Failed ID {illust.id}: {e}"

    # Execute downloads with progress bar
    results = []
    with tqdm(total=len(illusts), desc="Downloading") as pbar:
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            futures = [executor.submit(download_single, illust) for illust in illusts]

            for future in concurrent.futures.as_completed(futures):
                result = future.result()
                results.append(result)
                pbar.update(1)

    # Print summary
    successful = sum(1 for r in results if not r.startswith("Failed"))
    print(f"\nCompleted: {successful}/{len(illusts)} downloads successful")

    return results
```

### 2. Batch User Data Collection

```python
def collect_multiple_users_data(user_ids):
    """Collect data from multiple users"""

    all_data = {}

    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
        # Submit all user detail requests
        future_to_user = {
            executor.submit(api.user_detail, user_id): user_id
            for user_id in user_ids
        }

        for future in concurrent.futures.as_completed(future_to_user):
            user_id = future_to_user[future]
            try:
                user_detail = future.result()
                user_data = {
                    'profile': user_detail.user,
                    'stats': {
                        'followers': user_detail.total_followers,
                        'following': user_detail.total_following,
                        'illusts': user_detail.total_illusts,
                        'novels': user_detail.total_novels
                    }
                }
                all_data[user_id] = user_data
                print(f"Collected data for user {user_id}")

            except Exception as e:
                print(f"Failed to get data for user {user_id}: {e}")

    return all_data
```

## Novel Operations

### 1. Novel Content Retrieval

```python
def get_novel_content(novel_id):
    """Get formatted novel content"""

    # Get novel metadata
    novel_info = api.novel_detail(novel_id)
    novel = novel_info.novel

    # Get novel content
    webview_data = api.webview_novel(novel_id)

    print(f"Title: {novel.title}")
    print(f"Author: {novel.user.name}")
    print(f"Characters: {novel.text_length}")
    print(f"Created: {novel.create_date}")
    print("-" * 40)

    # Print content
    print(webview_data.text)

    # Save to file
    filename = f"{novel_id}_{novel.title.replace('/', '_')}.txt"
    with open(filename, 'w', encoding='utf-8') as f:
        f.write(f"Title: {novel.title}\n")
        f.write(f"Author: {novel.user.name}\n")
        f.write(f"URL: https://www.pixiv.net/novel/show.php?id={novel_id}\n")
        f.write("-" * 40 + "\n\n")
        f.write(webview_data.text)

    print(f"\nSaved to {filename}")
    return webview_data
```

### 2. Novel Series Collection

```python
def collect_novel_series(series_id):
    """Collect all novels in a series"""

    # Get series info
    series_info = api.novel_series(series_id)
    series = series_info.series

    print(f"Series: {series.title}")
    print(f"Novels: {series.total_novels}")
    print(f"Status: {'Complete' if series.is_concluded else 'Ongoing'}")
    print()

    all_novels = []
    page = 1

    while True:
        # Get novels in series (Pixiv doesn't have direct pagination for series)
        # This is a workaround - we'd need to search by series tag
        result = api.search_novel(
            word=f"シリーズ「{series.title}」",
            sort="date_asc"  # Oldest first
        )

        if not result.novels:
            break

        # Filter by series ID
        series_novels = [
            n for n in result.novels
            if n.series_id == series_id
        ]

        if not series_novels:
            break

        all_novels.extend(series_novels)
        print(f"Found {len(series_novels)} novels (page {page})")

        page += 1
        if len(all_novels) >= series.total_novels:
            break

    # Sort by publication date
    all_novels.sort(key=lambda x: x.create_date)

    # Download all novels
    for novel in all_novels:
        print(f"\nDownloading: {novel.title}")
        try:
            get_novel_content(novel.id)
        except Exception as e:
            print(f"Failed: {e}")

    return all_novels
```

## Error Handling Patterns

### 1. Robust API Wrapper

```python
from functools import wraps
import time

def robust_api_call(max_retries=3, delay=1):
    """Decorator for robust API calls with retry logic"""

    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(max_retries):
                try:
                    return func(*args, **kwargs)

                except PixivRateLimitError as e:
                    # Handle rate limiting
                    if e.retry_after:
                        wait_time = float(e.retry_after)
                        print(f"Rate limited. Waiting {wait_time} seconds...")
                        time.sleep(wait_time)
                    else:
                        time.sleep(delay * (2 ** attempt))

                except PixivAuthError:
                    # Try refreshing token
                    print("Auth error, refreshing token...")
                    try:
                        api.auth(refresh_token=refresh_token)
                        continue
                    except:
                        print("Token refresh failed")
                        raise

                except PixivNetworkError as e:
                    print(f"Network error (attempt {attempt + 1}): {e}")
                    if attempt < max_retries - 1:
                        time.sleep(delay * (2 ** attempt))

                except Exception as e:
                    print(f"Unexpected error: {e}")
                    raise

            raise Exception(f"Failed after {max_retries} attempts")

        return wrapper
    return decorator

# Usage
@robust_api_call(max_retries=3)
def safe_get_user_illusts(user_id):
    return api.user_illusts(user_id)
```

### 2. Graceful Degradation

```python
def get_user_data_with_fallback(user_id):
    """Get user data with fallback options"""

    try:
        # Try full user detail
        detail = api.user_detail(user_id)
        return detail

    except PixivNotFoundError:
        print(f"User {user_id} not found")
        return None

    except (PixivNetworkError, PixivRateLimitError) as e:
        print(f"API error for user {user_id}: {e}")

        # Fallback: try to get limited data from search
        try:
            search_result = api.search_user(str(user_id))
            if search_result.user_previews:
                return search_result.user_previews[0].user
        except:
            pass

        print(f"Could not retrieve data for user {user_id}")
        return None
```

## Rust Implementation Patterns

### 1. Builder Pattern for API Client

```rust
use pixiv_rs::PixivClient;

let client = PixivClient::builder()
    .with_refresh_token("YOUR_TOKEN")
    .with_user_agent("MyApp/1.0")
    .with_proxy("http://proxy:8080")
    .with_bypass(true)
    .build()?;

// Make API calls
let result = client
    .illust()
    .detail(59580629)
    .await?;
```

### 2. Stream-based Pagination

```rust
use futures::StreamExt;

let mut stream = client
    .search()
    .illust("landscape")
    .sort(SortOrder::Popular)
    .stream();

while let Some(Ok(illust)) = stream.next().await {
    println!("Title: {}", illust.title);
}
```

### 3. Concurrent Operations

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();

for id in illust_ids {
    let client = client.clone();
    set.spawn(async move {
        client.illust().detail(id).await
    });
}

while let Some(result) = set.join_next().await {
    match result {
        Ok(Ok(illust)) => println!("Got: {}", illust.title),
        Ok(Err(e)) => eprintln!("Error: {}", e),
        Err(e) => eprintln!("Join error: {}", e),
    }
}
```

## Best Practices

1. **Always handle rate limits** properly with exponential backoff
2. **Use generators** for memory-efficient pagination
3. **Implement proper error handling** for all API calls
4. **Cache responses** when appropriate to reduce API calls
5. **Use concurrent downloads** for batch operations
6. **Validate inputs** before making API calls
7. **Respect rate limits** and don't spam the API
8. **Use appropriate timeouts** for network operations
9. **Log errors** with sufficient context for debugging
10. **Consider the user experience** with progress indicators for long operations