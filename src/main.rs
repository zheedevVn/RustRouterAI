#![allow(non_snake_case)]

mod api;
mod models;
mod components;
mod views;
mod theme;

use dioxus::prelude::*;
use models::User;
use components::{TopHeader, BottomNavBar, CurrentTab};
use views::*;
use theme::MAIN_CSS;

fn main() {
    // Launch native mobile Dioxus app
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_user = use_signal(|| Option::<User>::None);
    let mut active_tab = use_signal(|| CurrentTab::Dashboard);

    let on_logout = move |_| {
        current_user.set(None);
        api::ApiClient::clear_token();
    };

    let user_clone = current_user();

    rsx! {
        // Embed global responsive styling
        style { "{MAIN_CSS}" }

        div { class: "app-container",
            if let Some(user) = user_clone {
                // Logged in UI
                TopHeader {
                    user: Some(user.clone()),
                    on_logout: on_logout,
                }

                div { class: "main-viewport",
                    match active_tab() {
                        CurrentTab::Dashboard => rsx! { DashboardView { user: Some(user) } },
                        CurrentTab::Accounts => rsx! { AccountsView {} },
                        CurrentTab::Groups => rsx! { GroupsView {} },
                        CurrentTab::Campaigns => rsx! { CampaignsView {} },
                        CurrentTab::Plans => rsx! { PlansView {} },
                        CurrentTab::Settings => rsx! { SettingsView {} },
                        _ => rsx! { DashboardView { user: Some(user) } },
                    }
                }

                BottomNavBar {
                    active_tab: active_tab(),
                    on_select: move |tab| active_tab.set(tab),
                }
            } else {
                // Login / Register View
                LoginView {
                    on_login_success: move |u| current_user.set(Some(u)),
                }
            }
        }
    }
}
