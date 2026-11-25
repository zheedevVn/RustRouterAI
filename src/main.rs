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
    // Launch native mobile Dioxus app with iOS-specific viewport and safe area shell
    let cfg = dioxus::mobile::Config::default()
        .with_custom_index(r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>RouterAI</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
        <meta name="apple-mobile-web-app-capable" content="yes">
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
        <meta name="format-detection" content="telephone=no">
        <!-- CUSTOM HEAD -->
    </head>
    <body style="margin: 0; padding: 0; background-color: #1e1f22; width: 100%; height: 100%; overflow: hidden; position: fixed; inset: 0;">
        <div id="main" style="width: 100%; height: 100%; display: flex; flex-direction: column;"></div>
        <!-- MODULE LOADER -->
    </body>
</html>"#.to_string());

    LaunchBuilder::mobile().with_cfg(cfg).launch(App);
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
