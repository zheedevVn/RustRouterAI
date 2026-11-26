use dioxus::prelude::*;
use crate::models::Campaign;

#[component]
pub fn CampaignsView() -> Element {
    let dummy_campaigns = vec![
        Campaign {
            id: "camp_1".to_string(),
            name: "Chiến dịch Chào mừng Thành Viên Mới".to_string(),
            status: "Hoạt động".to_string(),
            target_count: Some(500),
            sent_count: Some(182),
            success_rate: Some(98.5),
            created_at: Some("25/11/2026".to_string()),
        }
    ];

    rsx! {
        div { class: "content-scroll",
            div { class: "card",
                div { class: "card-title",
                    span { "QUẢN LÝ CHIẾN DỊCH" }
                    button {
                        class: "btn-primary",
                        style: "width: auto; height: 32px; padding: 0 12px; font-size: 12px;",
                        "+ Tạo Chiến Dịch"
                    }
                }

                div { style: "display: flex; flex-direction: column; gap: 10px;",
                    for c in dummy_campaigns {
                        div { class: "card", style: "background: #313338; margin-bottom: 0; padding: 14px;",
                            div { style: "display: flex; justify-content: space-between; align-items: flex-start;",
                                div {
                                    div { style: "font-weight: 600; color: #fff; font-size: 14px;", "{c.name}" }
                                    div { style: "font-size: 11px; color: #949ba4; margin-top: 2px;", "Tạo lúc: {c.created_at.unwrap_or_default()}" }
                                }
                                span { class: "status-pill", "{c.status}" }
                            }

                            div { style: "display: flex; gap: 16px; margin-top: 12px; font-size: 12px; color: #dbdee1;",
                                span { "Mục tiêu: ", strong { "{c.target_count.unwrap_or(0)}" } }
                                span { "Đã rải: ", strong { style: "color: #52c41a;", "{c.sent_count.unwrap_or(0)}" } }
                                span { "Tỉ lệ: ", strong { style: "color: #1890ff;", "{c.success_rate.unwrap_or(0.0)}%" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
