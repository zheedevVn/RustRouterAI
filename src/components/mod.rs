pub mod icons;

use dioxus::prelude::*;
use crate::models::User;
use icons::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentTab {
    Dashboard,
    Accounts,
    Groups,
    Campaigns,
    History,
    Plans,
    Settings,
}

use crate::theme::LOGO_SMALL_DATA_URI;

#[component]
pub fn TopHeader(user: Option<User>, on_logout: EventHandler<()>) -> Element {
    let username = user.as_ref().map(|u| u.username.clone()).unwrap_or_else(|| "Thành viên".to_string());

    rsx! {
        header { class: "top-header",
            div { class: "brand-badge",
                img { src: "{LOGO_SMALL_DATA_URI}", alt: "RouterAI" }
                span { "RouterAI" }
            }

            div { style: "display: flex; align-items: center; gap: 12px;",
                span { class: "status-pill",
                    span { style: "width: 6px; height: 6px; border-radius: 50%; background: #52c41a;" }
                    "Online"
                }

                span { style: "font-size: 13px; color: #dbdee1; font-weight: 500;",
                    "{username}"
                }

                button {
                    onclick: move |_| on_logout.call(()),
                    style: "background: transparent; border: none; color: #fa777c; cursor: pointer; display: flex; align-items: center; padding: 4px;",
                    title: "Đăng xuất",
                    IconLogout {}
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
                span { "Tổng Quan" }
            }

            div {
                class: if active_tab == CurrentTab::Accounts { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Accounts),
                IconAccounts {}
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
                class: if active_tab == CurrentTab::Plans { "nav-item active" } else { "nav-item" },
                onclick: move |_| on_select.call(CurrentTab::Plans),
                IconPlans {}
                span { "Gói VIP" }
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
