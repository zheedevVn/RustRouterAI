#![allow(non_snake_case)]

mod api;
mod models;
mod components;
mod views;
mod theme;

use dioxus::prelude::*;
use models::User;
use components::{TopHeader, SidebarDrawer, BottomNavBar, CurrentTab};
use views::*;
use theme::MAIN_CSS;

#[cfg(target_os = "ios")]
fn get_screen_bounds() -> (f64, f64) {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGPoint {
        x: f64,
        y: f64,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGSize {
        width: f64,
        height: f64,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGRect {
        origin: CGPoint,
        size: CGSize,
    }

    unsafe impl objc::Encode for CGPoint {
        fn encode() -> objc::Encoding {
            unsafe { objc::Encoding::from_str("{CGPoint=dd}") }
        }
    }
    unsafe impl objc::Encode for CGSize {
        fn encode() -> objc::Encoding {
            unsafe { objc::Encoding::from_str("{CGSize=dd}") }
        }
    }
    unsafe impl objc::Encode for CGRect {
        fn encode() -> objc::Encoding {
            unsafe { objc::Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
        }
    }

    unsafe {
        use objc::{class, msg_send, sel, sel_impl};
        let screen: *mut objc::runtime::Object = msg_send![class!(UIScreen), mainScreen];
        if !screen.is_null() {
            let bounds: CGRect = msg_send![screen, bounds];
            if bounds.size.width > 50.0 && bounds.size.height > 50.0 {
                return (bounds.size.width, bounds.size.height);
            }
        }
    }
    (393.0, 852.0)
}

#[cfg(not(target_os = "ios"))]
fn get_screen_bounds() -> (f64, f64) {
    (393.0, 852.0)
}

fn main() {
    let (screen_w, screen_h) = get_screen_bounds();

    // Explicitly configure WindowBuilder with native screen dimensions to prevent Dioxus desktop
    // from injecting a default 800x600 desktop window size on iOS devices.
    let window = dioxus::mobile::WindowBuilder::new()
        .with_title("RouterAI")
        .with_inner_size(dioxus::mobile::LogicalSize::new(screen_w, screen_h))
        .with_resizable(false);

    // Launch native mobile Dioxus app with iOS-specific viewport and safe area shell
    let cfg = dioxus::mobile::Config::default()
        .with_window(window)
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
    let mut is_sidebar_open = use_signal(|| false);

    let on_logout = move |_| {
        current_user.set(None);
        api::ApiClient::clear_token();
        is_sidebar_open.set(false);
    };

    let user_clone = current_user();

    rsx! {
        // Embed global responsive styling
        style { "{MAIN_CSS}" }

        div { class: "app-container",
            if let Some(user) = user_clone {
                // Top Header Bar
                TopHeader {
                    user: Some(user.clone()),
                    on_toggle_sidebar: move |_| is_sidebar_open.set(!is_sidebar_open()),
                    on_logout: on_logout,
                }

                // Opaque Sliding Drawer with Backdrop (eliminates transparent overlap bug)
                SidebarDrawer {
                    is_open: is_sidebar_open(),
                    active_tab: active_tab(),
                    user: Some(user.clone()),
                    on_select: move |tab| active_tab.set(tab),
                    on_close: move |_| is_sidebar_open.set(false),
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
