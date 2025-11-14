use pixiv_rs::{AppClient, HttpClient, RankingMode, Filter, SearchTarget, Sort, ContentType, FollowRestrict};

#[tokio::test]
async fn test_app_client_creation() {
    let http_client = HttpClient::new().unwrap();
    let app_client = AppClient::new(http_client);
    
    // 测试客户端创建
    assert_eq!(app_client.base_url(), "https://app-api.pixiv.net");
}

#[tokio::test]
async fn test_search_target_to_string() {
    assert_eq!(SearchTarget::PartialMatchForTags.to_string(), "partial_match_for_tags");
    assert_eq!(SearchTarget::ExactMatchForTags.to_string(), "exact_match_for_tags");
    assert_eq!(SearchTarget::TitleAndCaption.to_string(), "title_and_caption");
    assert_eq!(SearchTarget::Keyword.to_string(), "keyword");
}

#[tokio::test]
async fn test_sort_to_string() {
    assert_eq!(Sort::DateDesc.to_string(), "date_desc");
    assert_eq!(Sort::DateAsc.to_string(), "date_asc");
    assert_eq!(Sort::PopularDesc.to_string(), "popular_desc");
}

#[tokio::test]
async fn test_ranking_mode_to_string() {
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

#[tokio::test]
async fn test_content_type_to_string() {
    assert_eq!(ContentType::Illust.to_string(), "illust");
    assert_eq!(ContentType::Manga.to_string(), "manga");
}

#[tokio::test]
async fn test_filter_to_string() {
    assert_eq!(Filter::ForIOS.to_string(), "for_ios");
    assert_eq!(Filter::None.to_string(), "");
}

#[tokio::test]
async fn test_set_base_url() {
    let http_client = HttpClient::new().unwrap();
    let mut app_client = AppClient::new(http_client);
    
    app_client.set_base_url("https://example.com".to_string());
    assert_eq!(app_client.base_url(), "https://example.com");
}

#[tokio::test]
async fn test_follow_restrict_to_string() {
    assert_eq!(FollowRestrict::Public.to_string(), "public");
    assert_eq!(FollowRestrict::Private.to_string(), "private");
}