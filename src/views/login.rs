use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::User;
use crate::components::icons::*;

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
        let mut error_msg = error_msg;
        let mut notice_msg = notice_msg;
        let mut loading = loading;
        error_msg.set(None);
        notice_msg.set(None);
        loading.set(true);

        spawn(async move {
            let api = ApiClient::new();
            match api.get_oauth_info().await {
                Ok(info) => {
                    if let Some(cfg) = info.providers.get(provider) {
                        if cfg.configured {
                            if let Some(url) = &cfg.auth_url {
                                // Direct to OAuth URL
                                notice_msg.set(Some(format!("Chuyển hướng cổng {}: {}", cfg.name, url)));
                            }
                        } else {
                            notice_msg.set(Some(format!(
                                "Cổng đăng nhập {} chưa cấu hình Client ID & Secret trên server. Vui lòng đăng nhập bằng Mật khẩu ở trên!",
                                cfg.name
                            )));
                        }
                    }
                }
                Err(_) => {
                    notice_msg.set(Some(format!(
                        "Cổng đăng nhập {} yêu cầu cấu hình OAuth trên máy chủ để kích hoạt.",
                        provider
                    )));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; padding: 20px;",
            div {
                class: "card",
                style: "width: 100%; max-width: 380px; padding: 24px; text-align: center; margin-bottom: 0;",

                img {
                    src: "assets/Logo.png",
                    alt: "RouterAI",
                    style: "width: 200px; height: 38px; object-fit: contain; margin: 0 auto 20px auto; display: block;"
                }

                // Tab Switcher
                div {
                    style: "display: flex; background: #1e1f22; border-radius: 6px; padding: 3px; gap: 4px; margin-bottom: 16px;",
                    button {
                        style: if !is_register() {
                            "flex: 1; height: 34px; background: #1890ff; color: #fff; border: none; border-radius: 4px; font-weight: 600; font-size: 13px;"
                        } else {
                            "flex: 1; height: 34px; background: transparent; color: #949ba4; border: none; border-radius: 4px; font-size: 13px;"
                        },
                        onclick: move |_| {
                            is_register.set(false);
                            error_msg.set(None);
                        },
                        "Đăng Nhập"
                    }
                    button {
                        style: if is_register() {
                            "flex: 1; height: 34px; background: #1890ff; color: #fff; border: none; border-radius: 4px; font-weight: 600; font-size: 13px;"
                        } else {
                            "flex: 1; height: 34px; background: transparent; color: #949ba4; border: none; border-radius: 4px; font-size: 13px;"
                        },
                        onclick: move |_| {
                            is_register.set(true);
                            error_msg.set(None);
                        },
                        "Tạo Tài Khoản"
                    }
                }

                // Username / Email input
                div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "TÊN ĐĂNG NHẬP" }
                input {
                    class: "input-field",
                    r#type: "text",
                    placeholder: "Nhập username...",
                    value: "{username}",
                    oninput: move |e| username.set(e.value())
                }

                if is_register() {
                    div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "EMAIL (TÙY CHỌN)" }
                    input {
                        class: "input-field",
                        r#type: "email",
                        placeholder: "example@gmail.com",
                        value: "{email}",
                        oninput: move |e| email.set(e.value())
                    }
                }

                // Password input
                div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "MẬT KHẨU" }
                input {
                    class: "input-field",
                    r#type: "password",
                    placeholder: "Nhập mật khẩu...",
                    value: "{password}",
                    oninput: move |e| password.set(e.value())
                }

                if is_register() {
                    div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "XÁC NHẬN MẬT KHẨU" }
                    input {
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Nhập lại mật khẩu...",
                        value: "{confirm_password}",
                        oninput: move |e| confirm_password.set(e.value())
                    }
                }

                // Error / Notice Box
                if let Some(err) = error_msg() {
                    div {
                        style: "color: #fa777c; font-size: 12px; background: rgba(250, 119, 124, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(250, 119, 124, 0.25); margin-bottom: 12px;",
                        "{err}"
                    }
                }

                if let Some(notice) = notice_msg() {
                    div {
                        style: "color: #58a6ff; font-size: 12px; background: rgba(88, 166, 255, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(88, 166, 255, 0.25); margin-bottom: 12px; text-align: left;",
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
                div {
                    style: "display: flex; align-items: center; text-align: center; color: #80848e; font-size: 10px; font-weight: 600; letter-spacing: 0.5px; margin: 18px 0 12px 0;",
                    div { style: "flex: 1; border-bottom: 1px solid rgba(255, 255, 255, 0.08); margin-right: 8px;" }
                    span { "HOẶC TIẾP TỤC VỚI" }
                    div { style: "flex: 1; border-bottom: 1px solid rgba(255, 255, 255, 0.08); margin-left: 8px;" }
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px; margin-bottom: 14px;",
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("google"),
                        IconGoogle {}
                        "Google"
                    }
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("github"),
                        IconGithub {}
                        "GitHub"
                    }
                    button {
                        class: "btn-secondary",
                        style: "gap: 6px; font-size: 12px; height: 36px;",
                        onclick: move |_| handle_oauth("discord"),
                        IconDiscord {}
                        "Discord"
                    }
                }

                // Telegram Login Button
                button {
                    class: "btn-secondary",
                    style: "background: #229ED9; color: #fff; border: none; font-weight: 600; gap: 8px;",
                    onclick: move |_| handle_oauth("telegram"),
                    IconTelegram {}
                    "Đăng nhập qua Telegram"
                }

                div {
                    style: "margin-top: 16px; font-size: 11px; color: #80848e;",
                    "Copyright © 2026 RouterAI Team"
                }
            }
        }
    }
}
