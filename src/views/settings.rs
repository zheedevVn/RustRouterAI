use dioxus::prelude::*;
use crate::components::icons::*;

#[component]
pub fn SettingsView() -> Element {
    let mut active_sub_tab = use_signal(|| "oauth");
    let mut saved_msg = use_signal(|| Option::<String>::None);

    let mut google_id = use_signal(String::new);
    let mut google_secret = use_signal(String::new);
    let mut github_id = use_signal(String::new);
    let mut github_secret = use_signal(String::new);
    let mut discord_id = use_signal(String::new);
    let mut discord_secret = use_signal(String::new);

    let handle_save = move |_| {
        saved_msg.set(Some("Đã lưu cấu hình OAuth thành công!".to_string()));
    };

    rsx! {
        div { class: "content-scroll",
            div { class: "card",
                div { class: "card-title", "CÀI ĐẶT HỆ THỐNG" }

                // Sub-Tabs
                div {
                    style: "display: flex; gap: 6px; overflow-x: auto; white-space: nowrap; margin-bottom: 16px; padding-bottom: 4px;",
                    button {
                        style: if active_sub_tab() == "general" {
                            "padding: 6px 12px; background: #1890ff; color: #fff; border: none; border-radius: 6px; font-size: 12px; font-weight: 600;"
                        } else {
                            "padding: 6px 12px; background: #313338; color: #949ba4; border: 1px solid rgba(255,255,255,0.08); border-radius: 6px; font-size: 12px;"
                        },
                        onclick: move |_| active_sub_tab.set("general"),
                        "Cấu Hình Chung"
                    }
                    button {
                        style: if active_sub_tab() == "telegram" {
                            "padding: 6px 12px; background: #1890ff; color: #fff; border: none; border-radius: 6px; font-size: 12px; font-weight: 600;"
                        } else {
                            "padding: 6px 12px; background: #313338; color: #949ba4; border: 1px solid rgba(255,255,255,0.08); border-radius: 6px; font-size: 12px;"
                        },
                        onclick: move |_| active_sub_tab.set("telegram"),
                        "Telegram"
                    }
                    button {
                        style: if active_sub_tab() == "oauth" {
                            "padding: 6px 12px; background: #1890ff; color: #fff; border: none; border-radius: 6px; font-size: 12px; font-weight: 600;"
                        } else {
                            "padding: 6px 12px; background: #313338; color: #949ba4; border: 1px solid rgba(255,255,255,0.08); border-radius: 6px; font-size: 12px;"
                        },
                        onclick: move |_| active_sub_tab.set("oauth"),
                        "Cấu Hình OAuth"
                    }
                }

                if let Some(msg) = saved_msg() {
                    div {
                        style: "color: #52c41a; font-size: 12px; background: rgba(82, 196, 26, 0.1); padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(82, 196, 26, 0.25); margin-bottom: 12px;",
                        "{msg}"
                    }
                }

                if active_sub_tab() == "oauth" {
                    div {
                        p { style: "font-size: 12px; color: #949ba4; margin-bottom: 16px;",
                            "Khai báo Client ID và Client Secret từ các nhà cung cấp để kích hoạt đăng nhập một chạm cho thành viên."
                        }

                        // Google OAuth
                        div { style: "font-weight: 600; color: #ffffff; margin-bottom: 6px; display: flex; align-items: center; gap: 6px;",
                            IconGoogle {}
                            "Google OAuth 2.0"
                        }
                        input {
                            class: "input-field",
                            placeholder: "Google Client ID...",
                            value: "{google_id}",
                            oninput: move |e| google_id.set(e.value())
                        }
                        input {
                            class: "input-field",
                            r#type: "password",
                            placeholder: "Google Client Secret...",
                            value: "{google_secret}",
                            oninput: move |e| google_secret.set(e.value())
                        }

                        // GitHub OAuth
                        div { style: "font-weight: 600; color: #ffffff; margin-bottom: 6px; display: flex; align-items: center; gap: 6px; margin-top: 10px;",
                            IconGithub {}
                            "GitHub OAuth"
                        }
                        input {
                            class: "input-field",
                            placeholder: "GitHub Client ID...",
                            value: "{github_id}",
                            oninput: move |e| github_id.set(e.value())
                        }
                        input {
                            class: "input-field",
                            r#type: "password",
                            placeholder: "GitHub Client Secret...",
                            value: "{github_secret}",
                            oninput: move |e| github_secret.set(e.value())
                        }

                        // Discord OAuth
                        div { style: "font-weight: 600; color: #ffffff; margin-bottom: 6px; display: flex; align-items: center; gap: 6px; margin-top: 10px;",
                            IconDiscord {}
                            "Discord OAuth 2.0"
                        }
                        input {
                            class: "input-field",
                            placeholder: "Discord Client ID...",
                            value: "{discord_id}",
                            oninput: move |e| discord_id.set(e.value())
                        }
                        input {
                            class: "input-field",
                            r#type: "password",
                            placeholder: "Discord Client Secret...",
                            value: "{discord_secret}",
                            oninput: move |e| discord_secret.set(e.value())
                        }

                        button {
                            class: "btn-primary",
                            style: "margin-top: 16px;",
                            onclick: handle_save,
                            "Lưu Cấu Hình OAuth"
                        }
                    }
                } else {
                    div { style: "padding: 24px; text-align: center; color: #949ba4; font-size: 13px;",
                        "Cấu hình chung hệ thống và Telegram Bot Token."
                    }
                }
            }
        }
    }
}
