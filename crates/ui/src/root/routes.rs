use super::views::{Admin, Home, Layout, Page, RepoDetail};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/admin")]
        Admin {},
        #[route("/repo/:owner/:name")]
        RepoDetail { owner: String, name: String },
        #[route("/page")]
        Page {},
}
