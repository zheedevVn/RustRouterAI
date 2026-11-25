use dioxus::prelude::*;

#[component]
pub fn PlansView() -> Element {
    rsx! {
        div { class: "content-scroll",
            div { class: "card",
                div { class: "card-title", "BẢNG GIÁ & GÓI TÀI KHOẢN VIP" }
                p { style: "font-size: 12px; color: #949ba4; margin-bottom: 16px;",
                    "Chọn gói dịch vụ phù hợp với nhu cầu phát triển và mở rộng mạng lưới của bạn."
                }

                div { style: "display: flex; flex-direction: column; gap: 14px;",
                    // Gói Starter
                    div { class: "card", style: "background: #313338; border: 1px solid rgba(24, 144, 255, 0.3); margin-bottom: 0;",
                        div { style: "display: flex; justify-content: space-between; align-items: center;",
                            div {
                                div { style: "font-weight: 700; color: #fff; font-size: 15px;", "Gói Trải Nghiệm" }
                                div { style: "font-size: 11px; color: #949ba4;", "STARTER TIER" }
                            }
                            div { style: "font-size: 18px; font-weight: 800; color: #1890ff;", "99.000đ" }
                        }

                        div { style: "margin: 12px 0; font-size: 12px; color: #dbdee1; line-height: 1.8;",
                            div { "✓ Tối đa 2 nguồn tài khoản Telegram" }
                            div { "✓ Tối đa 5 mẫu tin nhắn rải" }
                            div { "✓ Cào tới 200 nhóm mục tiêu" }
                            div { "✓ Hỗ trợ tối đa 5 proxy" }
                        }

                        button { class: "btn-secondary", "Gia Hạn Gói Này" }
                    }

                    // Gói Doanh Nghiệp VIP
                    div { class: "card", style: "background: #313338; border: 1px solid rgba(82, 196, 26, 0.4); margin-bottom: 0;",
                        div { style: "display: flex; justify-content: space-between; align-items: center;",
                            div {
                                div { style: "font-weight: 700; color: #52c41a; font-size: 15px;", "Gói Doanh Nghiệp VIP" }
                                div { style: "font-size: 11px; color: #949ba4;", "BUSINESS UNLIMITED" }
                            }
                            div { style: "font-size: 18px; font-weight: 800; color: #52c41a;", "199.000đ" }
                        }

                        div { style: "margin: 12px 0; font-size: 12px; color: #dbdee1; line-height: 1.8;",
                            div { "✓ Tối đa 20 nguồn tài khoản Telegram" }
                            div { "✓ Tối đa 50 mẫu tin nhắn rải" }
                            div { "✓ Cào tới 10,000 nhóm mục tiêu" }
                            div { "✓ Không giới hạn proxy" }
                            div { "✓ Tùy chỉnh tốc độ rải tin & hỗ trợ 24/7" }
                        }

                        button { class: "btn-primary", "Đăng Ký Gói VIP" }
                    }
                }
            }
        }
    }
}
