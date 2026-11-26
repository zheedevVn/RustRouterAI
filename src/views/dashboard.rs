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
    let _display_name = user.as_ref().and_then(|u| u.full_name.clone()).unwrap_or_else(|| "Thành viên".to_string());

    rsx! {
        div { class: "content-scroll",
            // Quick Stats Grid
            div { class: "stats-grid",
                div { class: "stat-box",
                    div { class: "stat-label", "TÀI KHOẢN KẾT NỐI" }
                    div { class: "stat-value", "{current_stats.total_accounts}" }
                    div { style: "font-size: 11px; color: #52c41a; margin-top: 4px; font-weight: 500;", "● {current_stats.active_accounts} Hoạt động" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "NHÓM ĐÃ THU THẬP" }
                    div { class: "stat-value", "{current_stats.discovered_groups}" }
                    div { style: "font-size: 11px; color: #1890ff; margin-top: 4px; font-weight: 500;", "● {current_stats.pending_groups} Đang chờ" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "TỐC ĐỘ GỬI TIN" }
                    div { class: "stat-value", "{current_stats.crawl_speed_per_minute}" }
                    div { style: "font-size: 11px; color: #949ba4; margin-top: 4px;", "Links / phút" }
                }

                div { class: "stat-box",
                    div { class: "stat-label", "TRẠNG THÁI GATEWAY" }
                    div { class: "stat-value", style: "font-size: 15px; color: #52c41a; font-weight: 700;", "ONLINE" }
                    div { style: "font-size: 11px; color: #52c41a; margin-top: 4px; display: flex; align-items: center; gap: 4px;",
                        span { class: "live-indicator", style: "width: 6px; height: 6px;" }
                        "Đã đồng bộ"
                    }
                }
            }

            // Real-time Activity Line Chart Card (matching IMG_1483.png)
            div { class: "card",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 14px;",
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        span { class: "live-indicator" }
                        span { style: "font-weight: 600; color: #fff; font-size: 14px;", "Lịch sử gửi tin nhắn (Thời gian thực)" }
                    }
                    span { style: "font-size: 10px; color: #52c41a; font-weight: 700; background: rgba(82, 196, 26, 0.12); border: 1px solid rgba(82, 196, 26, 0.25); padding: 2px 8px; border-radius: 4px;",
                        "REALTIME"
                    }
                }

                div { style: "width: 100%; overflow-x: auto; background: #212429; border-radius: 8px; padding: 12px 6px 6px 6px; border: 1px solid rgba(255, 255, 255, 0.04);",
                    svg {
                        view_box: "0 0 520 180",
                        style: "width: 100%; height: auto; min-width: 480px; display: block;",

                        // SVG Definitions for gradient glow
                        defs {
                            linearGradient {
                                id: "greenAreaGrad",
                                x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                                stop { offset: "0%", stop_color: "#52c41a", stop_opacity: "0.35" }
                                stop { offset: "100%", stop_color: "#52c41a", stop_opacity: "0.0" }
                            }
                        }

                        // Horizontal Grid lines & Y-axis labels
                        line { x1: "35", y1: "30", x2: "505", y2: "30", stroke: "rgba(255, 255, 255, 0.06)", stroke_dasharray: "3 3" }
                        text { x: "25", y: "34", fill: "#949ba4", font_size: "10", text_anchor: "end", "20" }

                        line { x1: "35", y1: "80", x2: "505", y2: "80", stroke: "rgba(255, 255, 255, 0.06)", stroke_dasharray: "3 3" }
                        text { x: "25", y: "84", fill: "#949ba4", font_size: "10", text_anchor: "end", "10" }

                        line { x1: "35", y1: "130", x2: "505", y2: "130", stroke: "rgba(255, 255, 255, 0.1)" }
                        text { x: "25", y: "134", fill: "#949ba4", font_size: "10", text_anchor: "end", "0" }

                        // Gradient fill area
                        polygon {
                            points: "45,110 115,85 190,115 265,55 340,75 415,40 490,60 490,130 45,130",
                            fill: "url(#greenAreaGrad)",
                        }

                        // Main Trend Line
                        polyline {
                            points: "45,110 115,85 190,115 265,55 340,75 415,40 490,60",
                            fill: "none",
                            stroke: "#52c41a",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                        }

                        // Data Points (Dots)
                        circle { cx: "45", cy: "110", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "115", cy: "85", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "190", cy: "115", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "265", cy: "55", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "340", cy: "75", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "415", cy: "40", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }
                        circle { cx: "490", cy: "60", r: "4", fill: "#52c41a", stroke: "#212429", stroke_width: "2" }

                        // X-axis timestamps matching IMG_1483.png
                        text { x: "45", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:03:19" }
                        text { x: "115", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:03:44" }
                        text { x: "190", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:04:09" }
                        text { x: "265", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:04:34" }
                        text { x: "340", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:04:59" }
                        text { x: "415", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "11:05:24" }
                        text { x: "490", y: "155", fill: "#949ba4", font_size: "10", text_anchor: "middle", "Hiện tại" }
                    }
                }
            }

            // Donut Chart Card: "Tình trạng tài khoản" (matching IMG_1483.png)
            div { class: "card",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                    span { style: "font-weight: 600; color: #fff; font-size: 14px;", "Tình trạng tài khoản" }
                    span { style: "font-size: 11px; color: #949ba4;", "Tỷ lệ kích hoạt" }
                }

                div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 8px 0;",
                    svg {
                        view_box: "0 0 160 160",
                        style: "width: 140px; height: 140px; display: block;",

                        // Background track
                        circle {
                            cx: "80",
                            cy: "80",
                            r: "54",
                            fill: "none",
                            stroke: "#2b2d31",
                            stroke_width: "14",
                        }

                        // Active segment
                        circle {
                            cx: "80",
                            cy: "80",
                            r: "54",
                            fill: "none",
                            stroke: "#1890ff",
                            stroke_width: "14",
                            stroke_linecap: "round",
                            stroke_dasharray: "339.29",
                            stroke_dashoffset: if current_stats.total_accounts > 0 {
                                format!("{:.1}", 339.29 * (1.0 - (current_stats.active_accounts as f64 / current_stats.total_accounts as f64).min(1.0)))
                            } else {
                                "280.0".to_string()
                            },
                            transform: "rotate(-90 80 80)",
                        }

                        // Center counter
                        text {
                            x: "80",
                            y: "76",
                            text_anchor: "middle",
                            font_size: "20",
                            font_weight: "700",
                            fill: "#ffffff",
                            "{current_stats.active_accounts}/{current_stats.total_accounts}"
                        }
                        text {
                            x: "80",
                            y: "94",
                            text_anchor: "middle",
                            font_size: "11",
                            fill: "#949ba4",
                            "Tài khoản"
                        }
                    }

                    // Legend matching web
                    div { style: "display: flex; align-items: center; gap: 8px; margin-top: 10px;",
                        span { style: "display: inline-block; width: 10px; height: 10px; background: #1890ff; border-radius: 2px;" }
                        span { style: "font-size: 12px; color: #dbdee1; font-weight: 500;",
                            "Tài khoản {current_stats.active_accounts}/{current_stats.total_accounts}"
                        }
                    }
                }
            }

            // Quick Actions Bar
            div { class: "card",
                div { class: "card-title", "THAO TÁC NHANH" }
                div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px;",
                    button { class: "btn-secondary", style: "gap: 6px; font-size: 12px;",
                        IconAccounts {}
                        "+ Thêm Tài Khoản"
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
                        "Quản Lý Gói"
                    }
                }
            }
        }
    }
}
