use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::TelegramAccount;
use crate::components::icons::*;

#[component]
pub fn AccountsView() -> Element {
    let mut accounts = use_signal(Vec::<TelegramAccount>::new);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let api = ApiClient::new();
            if let Ok(data) = api.get_telegram_accounts().await {
                accounts.set(data);
            }
            loading.set(false);
        });
    });

    let account_list = accounts();

    rsx! {
        div { class: "content-scroll",
            div { class: "card",
                div { class: "card-title",
                    span { "NGUỒN TÀI KHOẢN TELEGRAM" }
                    button {
                        class: "btn-primary",
                        style: "width: auto; height: 32px; padding: 0 12px; font-size: 12px;",
                        "+ Thêm Tài Khoản"
                    }
                }

                if loading() {
                    div { style: "text-align: center; padding: 24px; color: #949ba4;", "Đang tải dữ liệu từ máy chủ..." }
                } else if account_list.is_empty() {
                    div { style: "text-align: center; padding: 32px 16px; background: #313338; border-radius: 8px;",
                        IconAccounts {}
                        p { style: "color: #949ba4; margin-top: 8px; font-size: 13px;", "Chưa có tài khoản Telegram nào được thêm." }
                        button {
                            class: "btn-primary",
                            style: "width: auto; margin: 12px auto 0 auto; height: 36px; padding: 0 16px; font-size: 12px;",
                            "Kết Nối Tài Khoản Đầu Tiên"
                        }
                    }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 10px;",
                        for acc in account_list {
                            div {
                                class: "card",
                                style: "background: #313338; margin-bottom: 0; padding: 12px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        strong { style: "color: #ffffff; font-size: 14px;", "{acc.phone}" }
                                        if let Some(un) = &acc.username {
                                            div { style: "font-size: 12px; color: #1890ff;", "@{un}" }
                                        }
                                    }
                                    span {
                                        class: "status-pill",
                                        "{acc.status}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
