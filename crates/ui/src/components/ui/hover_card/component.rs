use dioxus::prelude::*;
use dioxus_primitives::hover_card::{self, HoverCardContentProps, HoverCardTriggerProps};
use dioxus_sdk_time::use_debounce;

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,
    #[props(default)]
    pub default_open: bool,
    #[props(default)]
    pub on_open_change: Callback<bool>,
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(default = 90)]
    pub close_delay_ms: u64,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let mut internal_open = use_signal(|| props.default_open);
    let mut close_ticket = use_signal(|| 0u64);

    use_effect(move || {
        if let Some(next) = (props.open)() {
            internal_open.set(next);
        }
    });

    let mut delayed_close = use_debounce(
        std::time::Duration::from_millis(props.close_delay_ms.max(1)),
        move |ticket: u64| {
            if close_ticket() != ticket {
                return;
            }
            if internal_open() {
                internal_open.set(false);
                props.on_open_change.call(false);
            }
        },
    );

    let mut request_open = move || {
        close_ticket.with_mut(|t| *t += 1);
        if !internal_open() {
            internal_open.set(true);
            props.on_open_change.call(true);
        }
    };

    let mut request_close = move || {
        close_ticket.with_mut(|t| *t += 1);
        delayed_close.action(close_ticket());
    };

    let resolved_open = (props.open)().unwrap_or(internal_open());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        hover_card::HoverCard {
            class: "hover-card",
            open: resolved_open,
            default_open: props.default_open,
            on_open_change: move |next| {
                if (props.disabled)() {
                    return;
                }
                if next {
                    request_open();
                } else {
                    request_close();
                }
            },
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    rsx! {
        hover_card::HoverCardTrigger {
            class: "hover-card-trigger",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    rsx! {
        hover_card::HoverCardContent {
            class: "hover-card-content",
            side: props.side,
            align: props.align,
            id: props.id,
            force_mount: props.force_mount,
            attributes: props.attributes,
            {props.children}
        }
    }
}
