pub mod icons;

use dioxus::prelude::*;
use crate::models::User;
use icons::*;
use crate::theme::LOGO_SMALL_DATA_URI;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentTab {
    Dashboard,      // Trang Chủ
    Members,        // Quản Lý Thành Viên
    Accounts,       // Nguồn Tài Khoản
    Groups,         // Nhóm Telegram
    Campaigns,      // Quản Lý Chiến Dịch
    History,        // Lịch Sử Rải Tin Nhắn
    RentHistory,    // Lịch Sử Thuê Gói
    Plans,          // Quản Lý Gói
    Settings,       // Cài Đặt Hệ Thống
}

#[component]
pub fn TopHeader(
    user: Option<User>,
    on_toggle_sidebar: EventHandler<()>,
    on_logout: EventHandler<()>,
) -> Element {
    let username = user.as_ref().map(|u| u.username.clone()).unwrap_or_else(|| "Thành viên".to_string());
    let initial = username.chars().next().unwrap_or('U').to_ascii_uppercase();

    rsx! {
        header {
            style: "display: flex; align-items: center; justify-content: space-between; height: 50px; padding: 0 14px; background: #212429; border-bottom: 1px solid rgba(255, 255, 255, 0.08); flex-shrink: 0; z-index: 100;",

            // Left: Hamburger Menu + Logo
            div { style: "display: flex; align-items: center; gap: 10px;",
                button {
                    onclick: move |_| on_toggle_sidebar.call(()),
                    style: "background: transparent; border: none; color: #dbdee1; cursor: pointer; display: flex; align-items: center; justify-content: center; padding: 6px; border-radius: 6px;",
                    IconMenu {}
                }

                div { style: "display: flex; align-items: center; gap: 8px;",
                    img {
                        src: "{LOGO_SMALL_DATA_URI}",
                        alt: "RouterAI",
                        style: "height: 26px; object-fit: contain;"
                    }
                }
            }

            // Right: Notifications, Dark mode, User Avatar
            div { style: "display: flex; align-items: center; gap: 12px;",
                // Bell with notification badge
                div { style: "position: relative; display: flex; align-items: center; cursor: pointer; color: #949ba4;",
                    IconBell {}
                    span {
                        style: "position: absolute; top: -5px; right: -6px; background: #fa777c; color: #fff; font-size: 9px; font-weight: 700; border-radius: 10px; padding: 1px 4px; line-height: 1;",
                        "6"
                    }
                }

                // Dark mode moon icon
                div { style: "display: flex; align-items: center; color: #949ba4; cursor: pointer;",
                    span { style: "font-size: 14px;", "🌙" }
                }

                // User Avatar
                div {
                    style: "width: 28px; height: 28px; border-radius: 50%; background: #383a40; border: 1px solid rgba(255,255,255,0.15); display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 700; color: #1890ff; cursor: pointer;",
                    title: "{username}",
                    "{initial}"
                }
            }
        }
    }
}

#[component]
pub fn SidebarDrawer(
    is_open: bool,
    active_tab: CurrentTab,
    user: Option<User>,
    on_select: EventHandler<CurrentTab>,
    on_close: EventHandler<()>,
    on_logout: EventHandler<()>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let username = user.as_ref().map(|u| u.username.clone()).unwrap_or_else(|| "Thành viên".to_string());

    rsx! {
        // Dark backdrop to completely prevent content overlap/bleed-through
        div {
            style: "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px); z-index: 998; animation: fadeIn 0.2s ease;",
            onclick: move |_| on_close.call(()),
        }

        // Solid opaque sliding drawer
        aside {
            style: "position: fixed; top: 0; bottom: 0; left: 0; width: 270px; max-width: 80vw; background: #1e2126; border-right: 1px solid rgba(255, 255, 255, 0.08); z-index: 999; display: flex; flex-direction: column; box-shadow: 8px 0 32px rgba(0, 0, 0, 0.6); overflow-y: auto; animation: drawerSlideIn 0.22s cubic-bezier(0.16, 1, 0.3, 1);",

            // Header of Drawer
            div {
                style: "padding: 16px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid rgba(255, 255, 255, 0.06);",
                div { style: "display: flex; align-items: center; gap: 8px;",
                    img { src: "{LOGO_SMALL_DATA_URI}", alt: "RouterAI", style: "height: 24px;" }
                    span { style: "font-weight: 700; color: #fff; font-size: 15px;", "RouterAI" }
                }
                button {
                    onclick: move |_| on_close.call(()),
                    style: "background: transparent; border: none; color: #949ba4; font-size: 18px; cursor: pointer; padding: 4px;",
                    "✕"
                }
            }

            // Menu Items matching exactly the server web
            div { style: "flex: 1; padding: 10px 8px; display: flex; flex-direction: column; gap: 2px;",

                // 1. Trang Chủ
                div {
                    class: if active_tab == CurrentTab::Dashboard { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Dashboard); on_close.call(()); },
                    IconHome {}
                    span { "Trang Chủ" }
                }

                // 2. Quản Lý Thành Viên
                div {
                    class: if active_tab == CurrentTab::Members { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Members); on_close.call(()); },
                    IconMember {}
                    span { "Quản Lý Thành Viên" }
                }

                // 3. Nguồn Tài Khoản
                div {
                    class: if active_tab == CurrentTab::Accounts { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Accounts); on_close.call(()); },
                    IconSend {}
                    span { "Nguồn Tài Khoản" }
                }

                // 4. Nhóm Telegram
                div {
                    class: if active_tab == CurrentTab::Groups { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Groups); on_close.call(()); },
                    IconGroups {}
                    span { "Nhóm Telegram" }
                }

                // 5. Quản Lý Chiến Dịch
                div {
                    class: if active_tab == CurrentTab::Campaigns { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Campaigns); on_close.call(()); },
                    IconCampaigns {}
                    span { "Quản Lý Chiến Dịch" }
                }

                // 6. Lịch Sử Rải Tin Nhắn
                div {
                    class: if active_tab == CurrentTab::History { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::History); on_close.call(()); },
                    IconHistory {}
                    span { "Lịch Sử Rải Tin Nhắn" }
                }

                // 7. Lịch Sử Thuê Gói
                div {
                    class: if active_tab == CurrentTab::RentHistory { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::RentHistory); on_close.call(()); },
                    IconHistory {}
                    span { "Lịch Sử Thuê Gói" }
                }

                // 8. Quản Lý Gói
                div {
                    class: if active_tab == CurrentTab::Plans { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Plans); on_close.call(()); },
                    IconCrown {}
                    span { "Quản Lý Gói" }
                }

                // 9. Cài Đặt Hệ Thống
                div {
                    class: if active_tab == CurrentTab::Settings { "drawer-item active" } else { "drawer-item" },
                    onclick: move |_| { on_select.call(CurrentTab::Settings); on_close.call(()); },
                    IconSettings {}
                    span { "Cài Đặt Hệ Thống" }
                }
            }

            // Footer of Drawer: User Info + Logout
            div {
                style: "padding: 14px; border-top: 1px solid rgba(255, 255, 255, 0.06); display: flex; align-items: center; justify-content: space-between;",
                div { style: "display: flex; flex-direction: column;",
                    span { style: "font-size: 13px; font-weight: 600; color: #fff;", "{username}" }
                    span { style: "font-size: 11px; color: #52c41a;", "● Đang hoạt động" }
                }
                button {
                    onclick: move |_| on_logout.call(()),
                    style: "background: rgba(250, 119, 124, 0.1); border: 1px solid rgba(250, 119, 124, 0.25); color: #fa777c; border-radius: 6px; padding: 6px 10px; font-size: 12px; font-weight: 500; cursor: pointer;",
                    "Đăng Xuất"
                }
            }
        }
    }
}

#[component]
pub fn BottomNavBar(active_tab: CurrentTab, on_select: EventHandler<CurrentTab>) -> Element {
    rsx! {
        nav { class: "bottom-nav",
            div {
                class: if active_tab == CurrentTab::Dashboard { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Dashboard),
                IconHome {}
                span { "Trang Chủ" }
            }
            div {
                class: if active_tab == CurrentTab::Accounts { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Accounts),
                IconSend {}
                span { "Tài Khoản" }
            }
            div {
                class: if active_tab == CurrentTab::Groups { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Groups),
                IconGroups {}
                span { "Nhóm" }
            }
            div {
                class: if active_tab == CurrentTab::Campaigns { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Campaigns),
                IconCampaigns {}
                span { "Chiến Dịch" }
            }
            div {
                class: if active_tab == CurrentTab::Settings { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Settings),
                IconSettings {}
                span { "Cài Đặt" }
            }
        }
    }
}
