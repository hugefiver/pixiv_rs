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
    
    // Test unknown error code
    let unknown = ApiErrorCode::from("999");
    assert!(matches!(unknown, ApiErrorCode::Unknown(code) if code == "999"));
}

#[test]
fn test_api_error_code_display() {
    assert_eq!(format!("{}", ApiErrorCode::AuthError103), "Authentication error (103)");
    assert_eq!(format!("{}", ApiErrorCode::ServerError500), "Server error (500)");
    assert_eq!(format!("{}", ApiErrorCode::TooManyRequests429), "Too many requests (429)");
    assert_eq!(format!("{}", ApiErrorCode::NotFound404), "Not found (404)");
    assert_eq!(format!("{}", ApiErrorCode::Forbidden403), "Forbidden (403)");
    assert_eq!(format!("{}", ApiErrorCode::BadRequest400), "Bad request (400)");
    assert_eq!(format!("{}", ApiErrorCode::ServiceUnavailable503), "Service unavailable (503)");
    
    let unknown = ApiErrorCode::Unknown("999".to_string());
    assert_eq!(format!("{}", unknown), "Unknown error code: 999");
}

#[test]
fn test_api_error_details() {
    let details = ApiErrorDetails {
        code: ApiErrorCode::AuthError103,
        message: "Authentication failed".to_string(),
        headers: None,
        body: None,
    };
    
    assert!(matches!(details.code, ApiErrorCode::AuthError103));
    assert_eq!(details.message, "Authentication failed");
    assert!(details.headers.is_none());
    assert!(details.body.is_none());
}

#[test]
fn test_pixiv_error_display() {
    let api_error = PixivError::ApiError("API call failed".to_string());
    assert_eq!(format!("{}", api_error), "API error: API call failed");
    
    let auth_error = PixivError::AuthError("Authentication failed".to_string());
    assert_eq!(format!("{}", auth_error), "Authentication error: Authentication failed");
    
    let unknown_error = PixivError::Unknown("Unknown error".to_string());
    assert_eq!(format!("{}", unknown_error), "Unknown error: Unknown error");
    
    let details = ApiErrorDetails {
        code: ApiErrorCode::ServerError500,
        message: "Internal server error".to_string(),
        headers: None,
        body: None,
    };
    
    let api_error_with_details = PixivError::ApiErrorWithDetails { details };
    let display_str = format!("{}", api_error_with_details);
    assert!(display_str.contains("API error"));
    assert!(display_str.contains("ServerError500"));
}