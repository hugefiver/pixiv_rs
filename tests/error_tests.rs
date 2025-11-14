use pixiv_rs::error::{ApiErrorCode, ApiErrorDetails, PixivError};

#[test]
fn test_api_error_code_from_str() {
    assert!(matches!(ApiErrorCode::from("103"), ApiErrorCode::AuthError103));
    assert!(matches!(ApiErrorCode::from("500"), ApiErrorCode::ServerError500));
    assert!(matches!(ApiErrorCode::from("429"), ApiErrorCode::TooManyRequests429));
    assert!(matches!(ApiErrorCode::from("404"), ApiErrorCode::NotFound404));
    assert!(matches!(ApiErrorCode::from("403"), ApiErrorCode::Forbidden403));
    assert!(matches!(ApiErrorCode::from("400"), ApiErrorCode::BadRequest400));
    assert!(matches!(ApiErrorCode::from("503"), ApiErrorCode::ServiceUnavailable503));
    
    // 测试未知错误代码
    let unknown = ApiErrorCode::from("999");
    assert!(matches!(unknown, ApiErrorCode::Unknown(code) if code == "999"));
}

#[test]
fn test_api_error_code_display() {
    assert_eq!(format!("{}", ApiErrorCode::AuthError103), "认证错误 (103)");
    assert_eq!(format!("{}", ApiErrorCode::ServerError500), "服务器错误 (500)");
    assert_eq!(format!("{}", ApiErrorCode::TooManyRequests429), "请求过多 (429)");
    assert_eq!(format!("{}", ApiErrorCode::NotFound404), "未找到 (404)");
    assert_eq!(format!("{}", ApiErrorCode::Forbidden403), "禁止访问 (403)");
    assert_eq!(format!("{}", ApiErrorCode::BadRequest400), "错误请求 (400)");
    assert_eq!(format!("{}", ApiErrorCode::ServiceUnavailable503), "服务不可用 (503)");
    
    let unknown = ApiErrorCode::Unknown("999".to_string());
    assert_eq!(format!("{}", unknown), "未知错误代码: 999");
}

#[test]
fn test_api_error_details() {
    let details = ApiErrorDetails {
        code: ApiErrorCode::AuthError103,
        message: "认证失败".to_string(),
        headers: None,
        body: None,
    };
    
    assert!(matches!(details.code, ApiErrorCode::AuthError103));
    assert_eq!(details.message, "认证失败");
    assert!(details.headers.is_none());
    assert!(details.body.is_none());
}

#[test]
fn test_pixiv_error_display() {
    let api_error = PixivError::ApiError("API调用失败".to_string());
    assert_eq!(format!("{}", api_error), "API 错误: API调用失败");
    
    let auth_error = PixivError::AuthError("认证失败".to_string());
    assert_eq!(format!("{}", auth_error), "认证错误: 认证失败");
    
    let unknown_error = PixivError::Unknown("未知错误".to_string());
    assert_eq!(format!("{}", unknown_error), "未知错误: 未知错误");
    
    let details = ApiErrorDetails {
        code: ApiErrorCode::ServerError500,
        message: "服务器内部错误".to_string(),
        headers: None,
        body: None,
    };
    
    let api_error_with_details = PixivError::ApiErrorWithDetails { details };
    let display_str = format!("{}", api_error_with_details);
    assert!(display_str.contains("API 错误"));
    assert!(display_str.contains("ServerError500"));
}