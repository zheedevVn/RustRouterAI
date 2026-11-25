use serde::{Deserialize, Serialize};

/// Standard Server API Wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<T>,
}

/// User Profile Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default, rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default, rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub balance: Option<i64>,
}

/// Authentication Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub ok: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub jwt: Option<JwtToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}

/// Login Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: Option<String>,
    #[serde(default, rename = "deviceId")]
    pub device_id: String,
}

/// Register Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(default, rename = "deviceId")]
    pub device_id: String,
}

/// Telegram Account Entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramAccount {
    pub id: String,
    pub phone: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub proxy: Option<String>,
    #[serde(default, rename = "groupsCount")]
    pub groups_count: Option<i64>,
    #[serde(default, rename = "lastActive")]
    pub last_active: Option<String>,
}

/// Telegram Group Entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramGroup {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default, rename = "memberCount")]
    pub member_count: Option<i64>,
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "discoveredAt")]
    pub discovered_at: Option<String>,
}

/// Broadcast Campaign Entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(default, rename = "targetCount")]
    pub target_count: Option<i64>,
    #[serde(default, rename = "sentCount")]
    pub sent_count: Option<i64>,
    #[serde(default, rename = "successRate")]
    pub success_rate: Option<f64>,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<String>,
}

/// Subscription Plan Entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: i64,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub features: Vec<String>,
}

/// Broadcast History Item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryItem {
    pub id: String,
    #[serde(rename = "campaignName")]
    pub campaign_name: String,
    pub target: String,
    pub status: String,
    #[serde(rename = "sentAt")]
    pub sent_at: String,
}

/// OAuth Provider Configuration Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub name: String,
    pub configured: bool,
    #[serde(rename = "authUrl")]
    pub auth_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthInfoResponse {
    pub ok: bool,
    pub providers: std::collections::HashMap<String, OAuthProviderConfig>,
}

/// System Dashboard Statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardStats {
    pub total_accounts: i64,
    pub active_accounts: i64,
    pub discovered_groups: i64,
    pub pending_groups: i64,
    pub crawl_speed_per_minute: i64,
    pub system_status: String,
}
