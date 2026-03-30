use dioxus::prelude::*;

use crate::IO::admin::{
    create_backup as create_backup_api, delete_backup as delete_backup_api,
    list_backups as list_backups_api, restore_backup as restore_backup_api,
};

#[component]
pub fn Backup() -> Element {
    let mut refresh_nonce = use_signal(|| 0u32);
    let mut label = use_signal(String::new);
    let mut status = use_signal(|| None::<String>);

    let backups = use_server_future(move || {
        let _nonce = refresh_nonce();
        async move { list_backups_api().await }
    })?;

    let on_create = move |_| {
        let label_value = label();
        spawn(async move {
            match create_backup_api(Some(label_value)).await {
                Ok(item) => {
                    status.set(Some(format!("Created backup: {}", item.name)));
                    label.set(String::new());
                    refresh_nonce.with_mut(|v| *v += 1);
                }
                Err(err) => status.set(Some(format!("Create failed: {err}"))),
            }
        });
    };

    rsx! {
        section { class: "h-full min-h-0 w-full overflow-x-hidden overflow-y-auto space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5",
                    "BACKUP / DATABASE"
                }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3",
                    "Database Backups"
                }
                p { class: "text-xs text-secondary-5",
                    "创建数据库备份并管理历史备份。"
                }
            }

            div { class: "flex flex-col gap-2 md:flex-row",
                input {
                    class: "w-full border border-secondary-2 bg-primary px-3 py-2 text-sm",
                    placeholder: "backup label, e.g. pre_migration",
                    value: "{label}",
                    oninput: move |evt| label.set(evt.value()),
                }
                button {
                    class: "inline-flex items-center justify-center border border-secondary-2 bg-secondary-2 px-4 py-2 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                    onclick: on_create,
                    "Create Backup"
                }
            }

            if let Some(message) = status() {
                div { class: "text-xs text-secondary-5", "{message}" }
            }

            div { class: "border-t border-dashed border-primary-6 pt-3" }

            match backups() {
                Some(Ok(items)) => rsx! {
                    if items.is_empty() {
                        div { class: "text-sm text-secondary-5", "No backups yet." }
                    } else {
                        div { class: "space-y-2",
                            for item in items {
                                div { class: "grid grid-cols-1 gap-2 border border-secondary-2 bg-primary-1 p-3 md:grid-cols-[1fr_auto]",
                                    div { class: "space-y-1 text-sm",
                                        div { class: "font-medium text-secondary-3", "{item.name}" }
                                        div { class: "text-xs text-secondary-5", "created_at: {item.created_at_utc}" }
                                        div { class: "text-xs text-secondary-5", "size: {item.size_bytes} bytes" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        button {
                                            class: "border border-secondary-2 bg-primary px-3 py-1 text-xs text-secondary-3 hover:bg-primary-2",
                                            onclick: {
                                                let status = status;
                                                let refresh_nonce = refresh_nonce;
                                                let name = item.name.clone();
                                                move |_| {
                                                    let mut status = status;
                                                    let mut refresh_nonce = refresh_nonce;
                                                    let name = name.clone();
                                                    spawn(async move {
                                                        match restore_backup_api(name.clone()).await {
                                                            Ok(_) => {
                                                                status.set(Some(format!("Restored backup: {name}")));
                                                                refresh_nonce.with_mut(|v| *v += 1);
                                                            }
                                                            Err(err) => status.set(Some(format!("Restore failed: {err}"))),
                                                        }
                                                    });
                                                }
                                            },
                                            "Restore"
                                        }
                                        button {
                                            class: "border border-secondary-2 bg-primary px-3 py-1 text-xs text-secondary-3 hover:bg-primary-2",
                                            onclick: {
                                                let status = status;
                                                let refresh_nonce = refresh_nonce;
                                                let name = item.name.clone();
                                                move |_| {
                                                    let mut status = status;
                                                    let mut refresh_nonce = refresh_nonce;
                                                    let name = name.clone();
                                                    spawn(async move {
                                                        match delete_backup_api(name.clone()).await {
                                                            Ok(_) => {
                                                                status.set(Some(format!("Deleted backup: {name}")));
                                                                refresh_nonce.with_mut(|v| *v += 1);
                                                            }
                                                            Err(err) => status.set(Some(format!("Delete failed: {err}"))),
                                                        }
                                                    });
                                                }
                                            },
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div { class: "text-sm text-red-700", "Load backups failed: {err}" }
                },
                None => rsx! {
                    div { class: "text-sm text-secondary-5", "Loading backups..." }
                },
            }
        }
    }
}
