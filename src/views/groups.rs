use dioxus::prelude::*;
use crate::api::ApiClient;
use crate::models::TelegramGroup;

#[component]
pub fn GroupsView() -> Element {
    let mut groups = use_signal(Vec::<TelegramGroup>::new);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let api = ApiClient::new();
            if let Ok(data) = api.get_groups().await {
                groups.set(data);
            }
            loading.set(false);
        });
    });

    let group_list = groups();

    rsx! {
        div { class: "content-scroll",
            div { class: "card",
                div { class: "card-title",
                    span { "DANH SÁCH NHÓM ĐÃ THU THẬP" }
                    button {
                        class: "btn-primary",
                        style: "width: auto; height: 32px; padding: 0 12px; font-size: 12px;",
                        "Quét Nhóm Mới"
                    }
                }

                if loading() {
                    div { style: "text-align: center; padding: 24px; color: #949ba4;", "Đang đồng bộ dữ liệu nhóm..." }
                } else if group_list.is_empty() {
                    div { style: "text-align: center; padding: 32px 16px; background: #313338; border-radius: 8px;",
                        p { style: "color: #949ba4; font-size: 13px;", "Chưa phát hiện nhóm nào theo từ khóa hiện tại." }
                    }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        for grp in group_list {
                            div { style: "padding: 10px 12px; background: #313338; border-radius: 6px; display: flex; justify-content: space-between; align-items: center;",
                                div {
                                    div { style: "font-weight: 600; color: #fff;", "{grp.title}" }
                                    if let Some(un) = &grp.username {
                                        div { style: "font-size: 12px; color: #1890ff;", "@{un}" }
                                    }
                                }
                                span { style: "font-size: 12px; color: #949ba4;",
                                    "{grp.member_count.unwrap_or(0)} thành viên"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
