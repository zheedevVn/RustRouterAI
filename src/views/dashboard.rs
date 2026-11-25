use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::{DashboardStats, User};
use crate::components::icons::*;

#[component]
pub fn DashboardView(user: Option<User>) -> Element {
    let mut stats = use_signal(DashboardStats::default);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let api = ApiClient::new();
            if let Ok(data) = api.get_dashboard_stats().await {
                stats.set(data);
            }
            loading.set(false);
        });
    });

    let current_stats = stats();
    let display_name = user.as_ref().and_then(|u| u.full_name.clone()).unwrap_or_else(|| "Thành viên".to_string());

    rsx! {
        div { class: "content-scroll",
            // Welcome Header Card
            div { class: "card",
                div { style: "display: flex; align-items: center; gap: 14px;",
                    div {
                        style: "width: 48px; height: 48px; border-radius: 50%; background: #383a40; display: flex; align-items: center; justify-content: center; font-size: 20px; font-weight: 700; color: #1890ff;",
                        "{display_name.chars().next().unwrap_or('U')}"
                    }
                    div {
                        h3 { style: "font-size: 16px; color: #ffffff; margin-bottom: 2px;", "Chào mừng trở lại, {display_name}" }
                        p { style: "font-size: 12px; color: #949ba4;", "Bảng điều khiển hệ thống tự động hóa RouterAI" }
                    }
                }
            }

            // Quick Stats Grid
            div { class: "stats-grid",
                div { class: "stat-box",
                    div { class: "stat-label", "TÀI KHOẢN KẾT NỐI" }
                    div { class: "stat-value", "{current_stats.total_accounts}" }
                    div { style: "font-size: 11px; color: #52c41a; margin-top: 4px;", "{current_stats.active_accounts} Hoạt động" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "NHÓM ĐÃ PHÁT HIỆN" }
                    div { class: "stat-value", "{current_stats.discovered_groups}" }
                    div { style: "font-size: 11px; color: #1890ff; margin-top: 4px;", "{current_stats.pending_groups} Đang chờ cào" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "TỐC ĐỘ TÌM KIẾM" }
                    div { class: "stat-value", "{current_stats.crawl_speed_per_minute}" }
                    div { style: "font-size: 11px; color: #949ba4; margin-top: 4px;", "Links / phút" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "TRẠNG THÁI HỆ THỐNG" }
                    div { class: "stat-value", style: "font-size: 15px; color: #52c41a;", "ONLINE" }
                    div { style: "font-size: 11px; color: #949ba4; margin-top: 4px;", "Gateway đã kết nối" }
                }
            }

            // Quick Actions
            div { class: "card",
                div { class: "card-title", "THAO TÁC NHANH" }
                div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px;",
                    button { class: "btn-secondary", style: "gap: 6px; font-size: 12px;",
                        IconAccounts {}
                        "Thêm Nick Telegram"
                    }
                    button { class: "btn-secondary", style: "gap: 6px; font-size: 12px;",
                        IconGroups {}
                        "Quét Nhóm Mới"
                    }
                    button { class: "btn-secondary", style: "gap: 6px; font-size: 12px;",
                        IconCampaigns {}
                        "Tạo Chiến Dịch"
                    }
                    button { class: "btn-secondary", style: "gap: 6px; font-size: 12px;",
                        IconPlans {}
                        "Nâng Cấp Gói"
                    }
                }
            }

            // Live Activity Feed
            div { class: "card",
                div { class: "card-title",
                    span { "HOẠT ĐỘNG HỆ THỐNG (LIVE)" }
                    span { style: "font-size: 11px; color: #52c41a; font-weight: 500;", "Realtime" }
                }
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    div { style: "padding: 8px 12px; background: #313338; border-radius: 6px; font-size: 12px; color: #dbdee1;",
                        "Hệ thống RouterAI đang ở trạng thái sẵn sàng thực hiện các chiến dịch cào và gửi tin tự động."
                    }
                    div { style: "padding: 8px 12px; background: #313338; border-radius: 6px; font-size: 12px; color: #949ba4;",
                        "Phiên bản iOS Rust Client v1.0.0 đã kết nối thành công với máy chủ."
                    }
                }
            }
        }
    }
}
