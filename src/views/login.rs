use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::User;
use crate::components::icons::*;
use crate::theme::LOGO_DATA_URI;

#[component]
pub fn LoginView(on_login_success: EventHandler<User>) -> Element {
    let mut is_register = use_signal(|| false);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut notice_msg = use_signal(|| Option::<String>::None);

    let handle_submit = move |_| {
        let u = username();
        let p = password();
        let is_reg = is_register();
        let conf_p = confirm_password();
        let em = email();

        if u.trim().is_empty() || p.is_empty() {
            error_msg.set(Some("Vui lòng điền đầy đủ tên đăng nhập và mật khẩu!".to_string()));
            return;
        }

        if is_reg && p != conf_p {
            error_msg.set(Some("Mật khẩu xác nhận không trùng khớp!".to_string()));
            return;
        }

        loading.set(true);
        error_msg.set(None);
        notice_msg.set(None);

        spawn(async move {
            let api = ApiClient::new();
            if is_reg {
                match api.register(&u, &p, if em.trim().is_empty() { None } else { Some(em) }, Some(u.clone())).await {
                    Ok(auth) => {
                        if let Some(user) = auth.user {
                            on_login_success.call(user);
                        } else {
                            error_msg.set(Some("Không tìm thấy thông tin tài khoản.".to_string()));
                        }
                    }
                    Err(e) => error_msg.set(Some(e)),
                }
            } else {
                match api.login_password(&u, &p).await {
                    Ok(auth) => {
                        if let Some(user) = auth.user {
                            on_login_success.call(user);
                        } else {
                            error_msg.set(Some("Đăng nhập thất bại.".to_string()));
                        }
                    }
                    Err(e) => error_msg.set(Some(e)),
                }
            }
            loading.set(false);
        });
    };

    let handle_oauth = move |provider: &'static str| {
        let mut notice_msg = notice_msg;
        notice_msg.set(Some(format!("Cổng {} đang bảo trì, vui lòng đăng nhập bằng Mật khẩu hoặc Telegram!", provider)));
    };

    rsx! {
        div {
            style: "flex: 1; display: flex; align-items: center; justify-content: center; width: 100%; min-height: 0; padding: 16px; overflow-y: auto; box-sizing: border-box; position: relative;",

            // Ambient glowing particle in the background
            div { class: "login-ambient-bg" }

            div {
                class: "login-card",

                // Embedded brand logo
                img {
                    src: "{LOGO_DATA_URI}",
                    alt: "RouterAI",
                    style: "width: 195px; height: 38px; object-fit: contain; margin: 0 auto 12px auto; display: block;"
                }

                // Tab Switcher: Đăng Nhập / Tạo Tài Khoản
                div { class: "tab-switcher",
                    button {
                        class: if !is_register() { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| {
                            is_register.set(false);
                            error_msg.set(None);
                            notice_msg.set(None);
                        },
                        "Đăng Nhập"
                    }
                    button {
                        class: if is_register() { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| {
                            is_register.set(true);
                            error_msg.set(None);
                            notice_msg.set(None);
                        },
                        "Tạo Tài Khoản"
                    }
                }

                // Form Inputs
                div { style: "width: 100%; text-align: left;",
                    span { class: "input-label", if !is_register() { "TÊN ĐĂNG NHẬP HOẶC EMAIL" } else { "TÊN ĐĂNG NHẬP *" } }
                    input {
                        class: "input-field",
                        r#type: "text",
                        placeholder: if !is_register() { "Nhập username hoặc email..." } else { "Nhập username..." },
                        value: "{username}",
                        oninput: move |e| username.set(e.value())
                    }

                    if is_register() {
                        span { class: "input-label", "EMAIL (TÙY CHỌN)" }
                        input {
                            class: "input-field",
                            r#type: "email",
                            placeholder: "example@gmail.com",
                            value: "{email}",
                            oninput: move |e| email.set(e.value())
                        }
                    }

                    span { class: "input-label", if !is_register() { "MẬT KHẨU" } else { "MẬT KHẨU *" } }
                    input {
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Nhập mật khẩu...",
                        value: "{password}",
                        oninput: move |e| password.set(e.value())
                    }

                    if is_register() {
                        span { class: "input-label", "XÁC NHẬN MẬT KHẨU *" }
                        input {
                            class: "input-field",
                            r#type: "password",
                            placeholder: "Nhập lại mật khẩu...",
                            value: "{confirm_password}",
                            oninput: move |e| confirm_password.set(e.value())
                        }
                    }
                }

                // Error / Notice Box
                if let Some(err) = error_msg() {
                    div {
                        style: "color: #fa777c; font-size: 12px; background: rgba(250, 119, 124, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(250, 119, 124, 0.25); width: 100%; box-sizing: border-box; text-align: center; margin-bottom: 4px;",
                        "{err}"
                    }
                }

                if let Some(notice) = notice_msg() {
                    div {
                        style: "color: #58a6ff; font-size: 12px; background: rgba(88, 166, 255, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(88, 166, 255, 0.25); width: 100%; box-sizing: border-box; text-align: center; margin-bottom: 4px;",
                        "{notice}"
                    }
                }

                // Submit Button
                button {
                    class: "btn-primary",
                    disabled: loading(),
                    onclick: handle_submit,
                    if loading() { "Đang xử lý..." } else if is_register() { "Tạo Tài Khoản Ngay" } else { "Đăng Nhập Ngay" }
                }

                // Social Login Section
                div { class: "divider-text",
                    span { "« HOẶC TIẾP TỤC VỚI »" }
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px; width: 100%;",
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("Google"),
                        IconGoogle {}
                        "Google"
                    }
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("GitHub"),
                        IconGithub {}
                        "GitHub"
                    }
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("Discord"),
                        IconDiscord {}
                        "Discord"
                    }
                }

                // Telegram Divider
                div { class: "divider-text",
                    span { "« HOẶC ĐĂNG NHẬP QUA TELEGRAM »" }
                }

                // Native Telegram Deep-link Button: Opens Telegram app directly
                a {
                    href: "tg://resolve?domain=csmmovn_bot&start=login",
                    class: "btn-telegram",
                    target: "_blank",
                    onclick: move |_| {
                        // Fallback script if deep link needs web trigger
                    },
                    IconTelegram {}
                    "Đăng nhập qua Telegram"
                }

                div {
                    style: "margin-top: 6px; font-size: 11px; color: #72767d; font-family: -apple-system, BlinkMacSystemFont, sans-serif;",
                    "Copyright © 2021 - 2026 by RouterAI Team"
                }
            }
        }
    }
}
