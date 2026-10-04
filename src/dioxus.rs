// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Color, Language, Size, Theme, Variant, base_code_style, base_content_style,
    base_copy_trigger_style, base_lang_badge_style, base_title_style, themed_block_style,
    themed_header_style, themed_pre_style,
};
use dioxus::prelude::*;

/// Props for the [`Code`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct CodeProps {
    /// The inline code content to display.
    #[props(default)]
    pub children: Element,

    /// Color palette of the inline code element.
    #[props(default = Color::Default)]
    pub color: Color,

    /// Visual style variant.
    #[props(default = Variant::Subtle)]
    pub variant: Variant,

    /// Size of the inline code element.
    #[props(default = Size::Sm)]
    pub size: Size,

    /// Additional CSS class names on the `<code>` element.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the `<code>` element.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the element.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label for screen readers.
    #[props(default)]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// An inline code snippet with configurable color, variant, and size.
///
/// Renders a `<code>` element styled with monospace font and the selected
/// color/variant combination.
///
/// # Accessibility
///
/// - Renders as `<code>`, providing implicit semantics for machine-readable content.
/// - An optional `aria_label` may be supplied for additional context.
///
/// # Examples
///
/// ```rust
/// use coders::dioxus::Code;
/// use coders::{Color, Size, Variant};
/// use dioxus::prelude::*;
///
/// fn InlineCode() -> Element {
///     rsx! {
///         Code {
///             color: Color::Accent,
///             variant: Variant::Subtle,
///             size: Size::Sm,
///             "console.log()"
///         }
///     }
/// }
/// ```
#[component]
pub fn Code(props: CodeProps) -> Element {
    let color_style = match props.variant {
        Variant::Solid => props.color.to_solid_style(),
        Variant::Subtle => props.color.to_subtle_style(),
        Variant::Outline => props.color.to_outline_style(),
        Variant::Surface => props.color.to_surface_style(),
        Variant::Plain => "",
        Variant::Custom(_) => "",
    };

    rsx! {
        code {
            id: props.id,
            class: "code {props.variant.to_class()} {props.color.to_class()} {props.size.to_class()} {props.class}",
            style: "{base_code_style()} {props.size.to_style()} {color_style} {props.style}",
            aria_label: props.aria_label,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Shared context provided by [`Block`] to all child components.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockContext {
    /// The raw code string stored in the code block.
    pub code: &'static str,
    /// The language of the code block.
    pub language: Language,
    /// The syntax-highlight theme.
    pub theme: Theme,
}

/// Props for the [`Header`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// Slot content, title, badges, copy button.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the header `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,
}

/// The header bar of a [`Block`], containing a title and action controls.
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let theme = try_consume_context::<BlockContext>()
        .map(|c| c.theme)
        .unwrap_or_default();
    rsx! {
        div {
            id: props.id,
            class: "code-block__header {props.class}",
            style: "{themed_header_style(theme)} {props.style}",
            {props.children}
        }
    }
}

/// Props for the [`Title`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    /// Title text or node.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the title `<span>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the title `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,
}

/// A title label inside a [`Header`].
#[component]
pub fn Title(props: TitleProps) -> Element {
    rsx! {
        span {
            id: props.id,
            class: "code-block__title {props.class}",
            style: "{base_title_style()} {props.style}",
            {props.children}
        }
    }
}

/// Props for the [`CopyTrigger`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct CopyTriggerProps {
    /// Label shown before copying.
    #[props(default = "Copy")]
    pub copy_label: &'static str,

    /// Label shown after a successful copy.
    #[props(default = "Copied!")]
    pub copied_label: &'static str,

    /// Duration in milliseconds before label resets.
    #[props(default = 2000)]
    pub reset_ms: u32,

    /// Additional CSS class names on the button.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the button.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label for screen readers.
    #[props(default = "Copy code to clipboard")]
    pub aria_label: &'static str,
}

/// A button that copies the [`Block`] code to the clipboard.
///
/// Reads [`BlockContext`] from the nearest [`Block`] ancestor.
///
/// # Accessibility
///
/// - Renders as `<button type="button">`.
/// - `aria-live="polite"` announces the copy confirmation.
#[component]
pub fn CopyTrigger(props: CopyTriggerProps) -> Element {
    let ctx = consume_context::<BlockContext>();
    let mut copied = use_signal(|| false);
    let reset_ms = props.reset_ms;

    let on_click = move |_| {
        let code = ctx.code;
        spawn(async move {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(code)).await;
            }
            copied.set(true);
            gloo_timers::future::TimeoutFuture::new(reset_ms).await;
            copied.set(false);
        });
    };

    let label = if copied() {
        props.copied_label
    } else {
        props.copy_label
    };

    rsx! {
        button {
            id: props.id,
            r#type: "button",
            class: "code-block__copy-trigger {props.class}",
            style: "{base_copy_trigger_style()} {props.style}",
            onclick: on_click,
            aria_label: props.aria_label,
            "aria-live": "polite",
            {label}
        }
    }
}

/// Props for the [`Content`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    /// Optional children rendered inside the content area.
    #[props(default)]
    pub children: Option<Element>,

    /// Additional CSS class names on the content `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,

    /// CSS class names on the inner `<pre>` element.
    #[props(default)]
    pub pre_class: &'static str,

    /// Inline CSS on the inner `<pre>` element.
    #[props(default)]
    pub pre_style: &'static str,
}

/// The scrollable code area inside a [`Block`].
///
/// When no children provided, renders the raw code from context in a `<pre>`.
#[component]
pub fn Content(props: ContentProps) -> Element {
    let ctx = consume_context::<BlockContext>();
    let hl = crate::common::highlight_code(ctx.code, ctx.language, ctx.theme);

    rsx! {
        div {
            id: props.id,
            class: "code-block__content {props.class}",
            style: "{base_content_style()} {props.style}",
            match props.children {
                Some(children) => rsx! { {children} },
                None => rsx! {
                    pre {
                        class: "code-block__pre {props.pre_class}",
                        style: "{themed_pre_style(ctx.theme)} {props.pre_style}",
                        tabindex: "0",
                        code {
                            class: ctx.language.to_class(),
                            dangerous_inner_html: hl,
                        }
                    }
                },
            }
        }
    }
}

/// Props for the [`LanguageBadge`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct LanguageBadgeProps {
    /// Additional CSS class names.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS.
    #[props(default)]
    pub style: &'static str,
}

/// A small language label in the [`Header`].
#[component]
pub fn LanguageBadge(props: LanguageBadgeProps) -> Element {
    let ctx = consume_context::<BlockContext>();
    rsx! {
        span {
            class: "code-block__lang {props.class}",
            style: "{base_lang_badge_style()} {props.style}",
            aria_label: "Language: {ctx.language.to_label()}",
            {ctx.language.to_label()}
        }
    }
}

/// Props for the [`Block`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct BlockProps {
    /// The raw code string to display and copy.
    pub code: &'static str,

    /// The programming language for syntax highlighting and the language badge.
    #[props(default)]
    pub language: Language,

    /// Syntax-highlight colour theme (default: [`Theme::OneDark`]).
    #[props(default)]
    pub theme: Theme,

    /// Child components.
    #[props(default)]
    pub children: Option<Element>,

    /// Additional CSS class names on the root `<figure>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the root `<figure>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label for the region.
    #[props(default = "Code block")]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A composable code-block container providing context and structure.
///
/// Provides [`BlockContext`] with `code` and `language` to all descendants.
/// When no children are provided, a [`Content`] is rendered automatically.
///
/// # Accessibility
///
/// - Renders as `<figure role="region">` with `aria-label` for landmark navigation.
/// - Inner `<pre>` uses `tabindex="0"` for keyboard scrolling.
///
/// # Examples
///
/// ```rust
/// use coders::dioxus::{Block, Header, Title, CopyTrigger, Content};
/// use coders::Language;
/// use dioxus::prelude::*;
///
/// fn MyBlock() -> Element {
///     rsx! {
///         Block {
///             code: "fn main() {{}}",
///             language: Language::Rust,
///             Header {
///                 Title { "main.rs" }
///                 CopyTrigger {}
///             }
///             Content {}
///         }
///     }
/// }
/// ```
#[component]
pub fn Block(props: BlockProps) -> Element {
    let ctx = BlockContext {
        code: props.code,
        language: props.language,
        theme: props.theme,
    };
    use_context_provider(|| ctx);

    rsx! {
        figure {
            id: props.id,
            class: "code-block {props.language.to_class()} {props.class}",
            style: "{themed_block_style(props.theme)} {props.style}",
            role: "region",
            aria_label: props.aria_label,
            "data-testid": props.data_testid,
            match props.children {
                Some(children) => rsx! { {children} },
                None => rsx! { Content {} },
            }
        }
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
