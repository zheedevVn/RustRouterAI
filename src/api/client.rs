use reqwest::{header, Client};
use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::models::*;

lazy_static! {
    static ref AUTH_TOKEN: RwLock<Option<String>> = RwLock::new(None);
}

pub struct ApiClient {
    base_url: String,
    http: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("RouterAI-iOS-Rust/1.0"));

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self {
            base_url: "https://ai.mmovn.space".to_string(),
            http,
        }
    }

    pub fn set_token(token: &str) {
        if let Ok(mut lock) = AUTH_TOKEN.write() {
            *lock = Some(token.to_string());
        }
    }

    pub fn clear_token() {
        if let Ok(mut lock) = AUTH_TOKEN.write() {
            *lock = None;
        }
    }

    pub fn get_token() -> Option<String> {
        AUTH_TOKEN.read().ok().and_then(|t| t.clone())
    }

    fn auth_header(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = Self::get_token() {
            req.header(header::AUTHORIZATION, format!("Bearer {}", token))
        } else {
            req
        }
    }

    /// Đăng nhập bằng tên tài khoản và mật khẩu
    pub async fn login_password(&self, username: &str, password: &str) -> Result<AuthResponse, String> {
        let url = format!("{}/api/v1/auth/email-login", self.base_url);
        let payload = LoginPayload {
            username: username.to_string(),
            password: Some(password.to_string()),
            device_id: "ios_rust_device".to_string(),
        };

        let res = self.http.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối máy chủ: {}", e))?;

        let status = res.status();
        let body = res.text().await.map_err(|e| format!("Lỗi đọc phản hồi: {}", e))?;

        if !status.is_success() {
            return Err(format!("Đăng nhập thất bại: {}", body));
        }

        let auth_res: AuthResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Lỗi phân tích dữ liệu: {}", e))?;

        if let Some(jwt) = &auth_res.jwt {
            Self::set_token(&jwt.token);
        }

        Ok(auth_res)
    }

    /// Đăng ký tài khoản mới với mật khẩu
    pub async fn register(
        &self,
        username: &str,
        password: &str,
        email: Option<String>,
        full_name: Option<String>,
    ) -> Result<AuthResponse, String> {
        let url = format!("{}/api/v1/auth/register", self.base_url);
        let payload = RegisterPayload {
            username: username.to_string(),
            password: password.to_string(),
            email,
            full_name,
            device_id: "ios_rust_device".to_string(),
        };

        let res = self.http.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối: {}", e))?;

        let body = res.text().await.map_err(|e| format!("Lỗi dữ liệu: {}", e))?;
        let auth_res: AuthResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Lỗi xử lý phản hồi: {}", e))?;

        if let Some(jwt) = &auth_res.jwt {
            Self::set_token(&jwt.token);
        }

        Ok(auth_res)
    }

    /// Kiểm tra trạng thái cổng OAuth từ server
    pub async fn get_oauth_info(&self) -> Result<OAuthInfoResponse, String> {
        let url = format!("{}/api/v1/auth/oauth-info", self.base_url);
        let res = self.http.get(&url)
            .send()
            .await
            .map_err(|e| format!("Lỗi truy vấn: {}", e))?;

        let body = res.text().await.map_err(|e| format!("Lỗi phản hồi: {}", e))?;
        serde_json::from_str(&body).map_err(|e| format!("Lỗi định dạng: {}", e))
    }

    /// Đăng nhập bằng mã Token đồng bộ từ Web (JWT Token)
    pub async fn login_with_token(&self, token: &str) -> Result<User, String> {
        Self::set_token(token.trim());
        self.get_me().await
    }

    /// Đăng nhập bằng dữ liệu Telegram Auth Widget
    pub async fn login_telegram_data(&self, mut val: serde_json::Value) -> Result<AuthResponse, String> {
        let url = format!("{}/api/v1/auth/login", self.base_url);
        if let Some(obj) = val.as_object_mut() {
            obj.insert("deviceId".to_string(), serde_json::Value::String("ios_rust_device".to_string()));
        }

        let res = self.http.post(&url)
            .json(&val)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối máy chủ: {}", e))?;

        let body = res.text().await.map_err(|e| format!("Lỗi đọc phản hồi: {}", e))?;
        let auth_res: AuthResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Lỗi xử lý phản hồi: {}", e))?;

        if let Some(jwt) = &auth_res.jwt {
            Self::set_token(&jwt.token);
        }

        Ok(auth_res)
    }

    /// Lấy thông tin tài khoản hiện tại
    pub async fn get_me(&self) -> Result<User, String> {
        let url = format!("{}/api/v1/auth/info", self.base_url);
        let req = self.http.get(&url);
        let res = self.auth_header(req)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối: {}", e))?;

        let body = res.text().await.map_err(|e| format!("Lỗi dữ liệu: {}", e))?;
        let parsed: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        
        if let Some(user_val) = parsed.get("user") {
            serde_json::from_value(user_val.clone()).map_err(|e| e.to_string())
        } else {
            Err("Không tìm thấy thông tin người dùng".to_string())
        }
    }

    /// Lấy thống kê tổng quan hệ thống
    pub async fn get_dashboard_stats(&self) -> Result<DashboardStats, String> {
        let url = format!("{}/api/v1/dashboard/stats", self.base_url);
        let req = self.http.get(&url);
        let res = self.auth_header(req)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối: {}", e))?;

        if res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            Ok(serde_json::from_str(&body).unwrap_or_else(|_| DashboardStats {
                total_accounts: 0,
                active_accounts: 0,
                discovered_groups: 0,
                pending_groups: 0,
                crawl_speed_per_minute: 0,
                system_status: "CONNECTED".to_string(),
            }))
        } else {
            Ok(DashboardStats {
                total_accounts: 0,
                active_accounts: 0,
                discovered_groups: 0,
                pending_groups: 0,
                crawl_speed_per_minute: 0,
                system_status: "STANDBY".to_string(),
            })
        }
    }

    /// Lấy danh sách tài khoản Telegram
    pub async fn get_telegram_accounts(&self) -> Result<Vec<TelegramAccount>, String> {
        let url = format!("{}/api/v1/accounts", self.base_url);
        let req = self.http.get(&url);
        let res = self.auth_header(req)
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối: {}", e))?;

        let body = res.text().await.map_err(|e| format!("Lỗi đọc danh sách: {}", e))?;
        let parsed: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Some(items) = parsed.get("data").or_else(|| parsed.get("accounts")) {
            serde_json::from_value(items.clone()).map_err(|e| e.to_string())
        } else {
            Ok(vec![])
        }
    }

    /// Lấy danh sách nhóm Telegram
    pub async fn get_groups(&self) -> Result<Vec<TelegramGroup>, String> {
        let url = format!("{}/api/v1/groups", self.base_url);
        let req = self.http.get(&url);
        let res = self.auth_header(req)
            .send()
            .await
            .map_err(|e| format!("Lỗi: {}", e))?;

        let body = res.text().await.map_err(|e| e.to_string())?;
        let parsed: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Some(items) = parsed.get("data") {
            serde_json::from_value(items.clone()).map_err(|e| e.to_string())
        } else {
            Ok(vec![])
        }
    }

    /// Đăng xuất khỏi hệ thống
    pub async fn logout(&self) -> Result<(), String> {
        let url = format!("{}/api/v1/auth/logout", self.base_url);
        let req = self.http.post(&url);
        let _ = self.auth_header(req).send().await;
        Self::clear_token();
        Ok(())
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}
