use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::User;
use crate::components::icons::*;
use crate::theme::LOGO_DATA_URI;

#[derive(Clone, Copy, PartialEq)]
enum AuthTab {
    Login,
    Register,
    TelegramSync,
}

#[component]
pub fn LoginView(on_login_success: EventHandler<User>) -> Element {
    let mut current_tab = use_signal(|| AuthTab::Login);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut token_input = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut notice_msg = use_signal(|| Option::<String>::None);

    let handle_submit = move |_| {
        let u = username();
        let p = password();
        let tab = current_tab();
        let conf_p = confirm_password();
        let em = email();

        if u.trim().is_empty() || p.is_empty() {
            error_msg.set(Some("Vui lòng điền đầy đủ tên đăng nhập và mật khẩu!".to_string()));
            return;
        }

        if tab == AuthTab::Register && p != conf_p {
            error_msg.set(Some("Mật khẩu xác nhận không trùng khớp!".to_string()));
            return;
        }

        loading.set(true);
        error_msg.set(None);
        notice_msg.set(None);

        spawn(async move {
            let api = ApiClient::new();
            if tab == AuthTab::Register {
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

    let handle_token_login = move |_| {
        let t = token_input();
        if t.trim().is_empty() {
            error_msg.set(Some("Vui lòng nhập hoặc dán mã Token JWT từ server RouterAI!".to_string()));
            return;
        }

        loading.set(true);
        error_msg.set(None);
        notice_msg.set(None);

        spawn(async move {
            let api = ApiClient::new();
            match api.login_with_token(&t).await {
                Ok(user) => {
                    on_login_success.call(user);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Mã Token không hợp lệ hoặc đã hết hạn: {}", e)));
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
                                notice_msg.set(Some(format!("Chuyển hướng cổng {}: {}", cfg.name, url)));
                            }
                        } else {
                            notice_msg.set(Some(format!(
                                "Cổng đăng nhập {} chưa bật trên server ai.mmovn.space. Vui lòng đăng nhập bằng Mật khẩu hoặc Telegram!",
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
            style: "flex: 1; display: flex; align-items: center; justify-content: center; width: 100%; min-height: 0; padding: 16px; overflow-y: auto; box-sizing: border-box;",
            div {
                class: "card",
                style: "width: 100%; max-width: 390px; padding: 22px 18px; text-align: center; margin: auto; box-sizing: border-box;",

                // Embedded high-resolution brand logo (guaranteed no 404 in iOS sandbox)
                img {
                    src: "{LOGO_DATA_URI}",
                    alt: "RouterAI",
                    style: "width: 190px; height: 36px; object-fit: contain; margin: 0 auto 16px auto; display: block;"
                }

                // Tab Switcher (Login / Register / Telegram Sync)
                div {
                    style: "display: flex; background: #1e1f22; border-radius: 6px; padding: 3px; gap: 4px; margin-bottom: 16px;",
                    button {
                        style: if current_tab() == AuthTab::Login {
                            "flex: 1; height: 32px; background: #1890ff; color: #fff; border: none; border-radius: 4px; font-weight: 600; font-size: 12px; cursor: pointer;"
                        } else {
                            "flex: 1; height: 32px; background: transparent; color: #949ba4; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;"
                        },
                        onclick: move |_| {
                            current_tab.set(AuthTab::Login);
                            error_msg.set(None);
                        },
                        "Đăng Nhập"
                    }
                    button {
                        style: if current_tab() == AuthTab::Register {
                            "flex: 1; height: 32px; background: #1890ff; color: #fff; border: none; border-radius: 4px; font-weight: 600; font-size: 12px; cursor: pointer;"
                        } else {
                            "flex: 1; height: 32px; background: transparent; color: #949ba4; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;"
                        },
                        onclick: move |_| {
                            current_tab.set(AuthTab::Register);
                            error_msg.set(None);
                        },
                        "Đăng Ký"
                    }
                    button {
                        style: if current_tab() == AuthTab::TelegramSync {
                            "flex: 1; height: 32px; background: #229ED9; color: #fff; border: none; border-radius: 4px; font-weight: 600; font-size: 12px; cursor: pointer;"
                        } else {
                            "flex: 1; height: 32px; background: transparent; color: #949ba4; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;"
                        },
                        onclick: move |_| {
                            current_tab.set(AuthTab::TelegramSync);
                            error_msg.set(None);
                        },
                        "Telegram / Token"
                    }
                }

                if current_tab() == AuthTab::TelegramSync {
                    // Telegram & Server Token Sync View
                    div {
                        style: "background: #25282e; border: 1px solid rgba(255,255,255,0.06); border-radius: 8px; padding: 14px; margin-bottom: 14px; text-align: left;",
                        
                        div {
                            style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                            span { style: "color: #229ED9;", IconTelegram {} }
                            strong { style: "color: #fff; font-size: 13px;", "Xác Thực Telegram RouterAI" }
                        }
                        
                        p {
                            style: "color: #949ba4; font-size: 12px; line-height: 1.5; margin-bottom: 12px;",
                            "Hệ thống RouterAI đồng bộ trực tiếp với Bot Telegram "
                            strong { style: "color: #229ED9;", "@csmmovn_bot" }
                            " và máy chủ "
                            strong { style: "color: #1890ff;", "ai.mmovn.space" }
                            "."
                        }

                        // Telegram Bot direct link
                        a {
                            href: "tg://resolve?domain=csmmovn_bot",
                            target: "_blank",
                            class: "btn-secondary",
                            style: "display: flex; align-items: center; justify-content: center; gap: 8px; background: #229ED9; color: #fff; border: none; font-weight: 600; text-decoration: none; height: 38px; margin-bottom: 8px;",
                            IconTelegram {}
                            "Mở Bot @csmmovn_bot trên Telegram"
                        }

                        // Web Server link
                        a {
                            href: "https://ai.mmovn.space/login",
                            target: "_blank",
                            class: "btn-secondary",
                            style: "display: flex; align-items: center; justify-content: center; gap: 8px; background: #313338; color: #dbdee1; border: 1px solid rgba(255,255,255,0.1); font-weight: 500; text-decoration: none; height: 36px; margin-bottom: 14px;",
                            "Mở trang đăng nhập ai.mmovn.space"
                        }

                        div { style: "border-top: 1px dashed rgba(255,255,255,0.1); margin: 12px 0;" }

                        div { style: "font-size: 11px; font-weight: 600; color: #58a6ff; margin-bottom: 4px;", "ĐỒNG BỘ TOKEN TỪ SERVER" }
                        p {
                            style: "font-size: 11px; color: #80848e; margin-bottom: 8px;",
                            "Dán mã Token (JWT) từ web hoặc từ Bot để đăng nhập tức thì:"
                        }
                        textarea {
                            class: "input-field",
                            style: "height: 60px; padding: 8px; font-size: 12px; font-family: monospace; resize: none;",
                            placeholder: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                            value: "{token_input}",
                            oninput: move |e| token_input.set(e.value())
                        }

                        button {
                            class: "btn-primary",
                            disabled: loading(),
                            onclick: handle_token_login,
                            if loading() { "Đang đồng bộ..." } else { "Đồng Bộ & Đăng Nhập Ngay" }
                        }
                    }
                } else {
                    // Username & Password Forms (Login / Register)
                    div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "TÊN ĐĂNG NHẬP" }
                    input {
                        class: "input-field",
                        r#type: "text",
                        placeholder: "Nhập username...",
                        value: "{username}",
                        oninput: move |e| username.set(e.value())
                    }

                    if current_tab() == AuthTab::Register {
                        div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "EMAIL (TÙY CHỌN)" }
                        input {
                            class: "input-field",
                            r#type: "email",
                            placeholder: "example@gmail.com",
                            value: "{email}",
                            oninput: move |e| email.set(e.value())
                        }
                    }

                    div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "MẬT KHẨU" }
                    input {
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Nhập mật khẩu...",
                        value: "{password}",
                        oninput: move |e| password.set(e.value())
                    }

                    if current_tab() == AuthTab::Register {
                        div { style: "text-align: left; margin-bottom: 4px; font-size: 11px; font-weight: 600; color: #949ba4;", "XÁC NHẬN MẬT KHẨU" }
                        input {
                            class: "input-field",
                            r#type: "password",
                            placeholder: "Nhập lại mật khẩu...",
                            value: "{confirm_password}",
                            oninput: move |e| confirm_password.set(e.value())
                        }
                    }

                    // Submit Button
                    button {
                        class: "btn-primary",
                        disabled: loading(),
                        onclick: handle_submit,
                        if loading() { "Đang xử lý..." } else if current_tab() == AuthTab::Register { "Tạo Tài Khoản Ngay" } else { "Đăng Nhập Ngay" }
                    }
                }

                // Error / Notice Box
                if let Some(err) = error_msg() {
                    div {
                        style: "color: #fa777c; font-size: 12px; background: rgba(250, 119, 124, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(250, 119, 124, 0.25); margin: 12px 0; text-align: left;",
                        "{err}"
                    }
                }

                if let Some(notice) = notice_msg() {
                    div {
                        style: "color: #58a6ff; font-size: 12px; background: rgba(88, 166, 255, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(88, 166, 255, 0.25); margin: 12px 0; text-align: left;",
                        "{notice}"
                    }
                }

                // Social Login Section
                div {
                    style: "display: flex; align-items: center; text-align: center; color: #80848e; font-size: 10px; font-weight: 600; letter-spacing: 0.5px; margin: 16px 0 10px 0;",
                    div { style: "flex: 1; border-bottom: 1px solid rgba(255, 255, 255, 0.08); margin-right: 8px;" }
                    span { "HOẶC TIẾP TỤC VỚI" }
                    div { style: "flex: 1; border-bottom: 1px solid rgba(255, 255, 255, 0.08); margin-left: 8px;" }
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px; margin-bottom: 12px;",
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

                // Telegram Switch Button
                button {
                    class: "btn-secondary",
                    style: "background: #229ED9; color: #fff; border: none; font-weight: 600; gap: 8px;",
                    onclick: move |_| {
                        current_tab.set(AuthTab::TelegramSync);
                        error_msg.set(None);
                        notice_msg.set(None);
                    },
                    IconTelegram {}
                    "Đăng nhập qua Telegram (@csmmovn_bot)"
                }

                div {
                    style: "margin-top: 14px; font-size: 11px; color: #80848e;",
                    "Copyright © 2026 RouterAI Team • ai.mmovn.space"
                }
            }
        }
    }
}
