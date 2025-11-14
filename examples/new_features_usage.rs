//! 新功能使用示例
//!
//! 本示例展示了Rust版Pixiv客户端库新增的API端点和辅助功能的使用方法。

use pixiv_rs::{
    AppClient, HttpClient, FollowRestrict, Filter, ContentType, 
    download, parse_qs, extract_extension, safe_filename
};
use std::path::Path;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    #[cfg(feature = "tracing-subscriber")]
    tracing_subscriber::fmt::init();
    
    #[cfg(not(feature = "tracing-subscriber"))]
    println!("警告: 未启用tracing-subscriber功能，日志可能不会显示");

    // 创建HTTP客户端和App客户端
    let http_client = HttpClient::new()?;
    let mut app_client = AppClient::new(http_client);

    // 设置访问令牌（实际使用时需要有效的令牌）
    // app_client.http_client.set_access_token("your_access_token".to_string());

    println!("=== Pixiv Rust客户端新功能示例 ===\n");

    // 1. 获取关注用户的插画
    println!("1. 获取关注用户的插画");
    match app_client.illust_follow(FollowRestrict::Public, Some(0)).await {
        Ok(response) => {
            println!("成功获取 {} 个关注用户的插画", response.illusts.len());
            if let Some(next_url) = &response.next_url {
                println!("下一页URL: {}", next_url);
            }
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 2. 获取插画评论
    println!("\n2. 获取插画评论");
    match app_client.illust_comments(12345678, Some(0), Some(true)).await {
        Ok(response) => {
            println!("成功获取 {} 条评论", response.comments.len());
            if let Some(total) = response.total_comments {
                println!("总评论数: {}", total);
            }
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 3. 获取用户关注列表
    println!("\n3. 获取用户关注列表");
    match app_client.user_following(12345678, FollowRestrict::Public, Some(0)).await {
        Ok(response) => {
            println!("成功获取 {} 个关注的用户", response.user_previews.len());
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 4. 获取用户粉丝列表
    println!("\n4. 获取用户粉丝列表");
    match app_client.user_follower(12345678, Filter::ForIOS, Some(0)).await {
        Ok(response) => {
            println!("成功获取 {} 个粉丝", response.user_previews.len());
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 5. 获取用户好P友列表
    println!("\n5. 获取用户好P友列表");
    match app_client.user_mypixiv(12345678, Some(0)).await {
        Ok(response) => {
            println!("成功获取 {} 个好P友", response.user_previews.len());
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 6. 添加插画收藏
    println!("\n6. 添加插画收藏");
    match app_client.illust_bookmark_add(
        12345678, 
        FollowRestrict::Public, 
        Some(vec!["标签1".to_string(), "标签2".to_string()])
    ).await {
        Ok(response) => {
            if response.success {
                println!("成功添加收藏");
            } else {
                println!("添加收藏失败: {:?}", response.error);
            }
        }
        Err(e) => println!("添加失败: {}", e),
    }

    // 7. 删除插画收藏
    println!("\n7. 删除插画收藏");
    match app_client.illust_bookmark_delete(12345678).await {
        Ok(response) => {
            if response.success {
                println!("成功删除收藏");
            } else {
                println!("删除收藏失败: {:?}", response.error);
            }
        }
        Err(e) => println!("删除失败: {}", e),
    }

    // 8. 获取趋势标签
    println!("\n8. 获取趋势标签");
    match app_client.trending_tags_illust(Filter::ForIOS).await {
        Ok(response) => {
            println!("成功获取 {} 个趋势标签", response.trend_tags.len());
            for tag in &response.trend_tags[..3.min(response.trend_tags.len())] {
                println!("标签: {} ({})", tag.tag, 
                    tag.translated_name.as_ref().unwrap_or(&tag.tag));
            }
        }
        Err(e) => println!("获取失败: {}", e),
    }

    // 9. 获取Ugoira元数据
    println!("\n9. 获取Ugoira元数据");
    match app_client.ugoira_metadata(12345678).await {
        Ok(response) => {
            let metadata = &response.ugoira_metadata;
            println!("Ugoira ID: {}", metadata.illust_id);
            println!("帧数: {}", metadata.frames.len());
            println!("MIME类型: {}", metadata.mime_type);
        }
        Err(e) => println!("获取失败: {}", e),
    }

    println!("\n=== 辅助功能示例 ===\n");

    // 10. 解析查询参数
    println!("10. 解析查询参数");
    let url = "https://app-api.pixiv.net/v1/illust/recommended?offset=20&limit=30&filter=for_ios";
    let params = parse_qs(url);
    println!("解析结果: {:?}", params);

    // 11. 提取文件扩展名
    println!("\n11. 提取文件扩展名");
    let image_urls = [
        "https://i.pximg.net/img-original/img/2023/01/01/00/00/00/12345678_p0.jpg",
        "https://i.pximg.net/img-original/img/2023/01/01/00/00/00/12345678_p0.png?size=large",
        "https://i.pximg.net/img-original/img/2023/01/01/00/00/00/12345678_p0",
    ];
    
    for url in &image_urls {
        match extract_extension(url) {
            Some(ext) => println!("URL: {} -> 扩展名: {}", url, ext),
            None => println!("URL: {} -> 无扩展名", url),
        }
    }

    // 12. 生成安全文件名
    println!("\n12. 生成安全文件名");
    let unsafe_names = [
        "test/file:name?.jpg",
        "normal_image.png",
        "illust|title<test>.jpg",
        "  spaced filename  ",
    ];
    
    for name in &unsafe_names {
        let safe = safe_filename(name);
        println!("不安全: '{}' -> 安全: '{}'", name, safe);
    }

    // 13. 下载图片（示例）
    println!("\n13. 下载图片示例");
    let http_client = HttpClient::new()?;
    let image_url = "https://i.pximg.net/img-original/img/2023/01/01/00/00/00/12345678_p0.jpg";
    let save_path = Path::new("downloaded_image.jpg");
    
    // 注意：实际下载需要有效的URL和访问令牌
    // 这里只展示函数调用方式
    println!("下载图片从: {} 到: {:?}", image_url, save_path);
    // match download(&http_client, image_url, save_path).await {
    //     Ok(()) => println!("下载成功"),
    //     Err(e) => println!("下载失败: {}", e),
    // }

    println!("\n=== 示例完成 ===");
    Ok(())
}