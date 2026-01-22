use dioxus::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ListItem {
    pub id: u32,
    pub title: String,
}

#[post("/fetch_page")]
async fn fetch_page(page: u32) -> Result<Vec<ListItem>, ServerFnError> {
    let items = (1..=10)
        .map(|i| ListItem {
            id: (page - 1) * 10 + i,
            title: format!("Item {}", (page - 1) * 10 + i),
        })
        .collect();

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(items)
}

#[component]
pub fn PaginatedList() -> Element {
    let mut current_page = use_signal(|| 1u32);

    let page_data = use_server_future(move || fetch_page(current_page()))?;

    rsx! {
        div { class: "paginated-list",
            h2 { "分页列表" }

            match page_data.cloned() {
                Some(Ok(data)) => rsx! {
                    div { class: "items",
                        for item in data {
                            div { class: "item", key: "{item.id}", "{item.id}: {item.title}" }
                        }
                    }

                    div { class: "pagination",
                        button {
                            onclick: move |_| {
                                if current_page() > 1 {
                                    current_page -= 1;
                                }
                            },
                            disabled: current_page() <= 1,
                            "上一页"
                        }

                        span { "第 {current_page} 页" }

                        button {
                            onclick: move |_| {
                                current_page += 1;
                            },
                            "下一页"
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div { "加载失败" }
                },
                None => rsx! {
                    div { "加载中..." }
                },
            }
        }
    }
}
