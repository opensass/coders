// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    Color, Language, Size, Theme, Variant, base_code_style, base_content_style,
    base_copy_trigger_style, base_lang_badge_style, base_title_style, themed_block_style,
    themed_header_style, themed_pre_style,
};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

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
/// use coders::leptos::Code;
/// use coders::{Color, Size, Variant};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn InlineCode() -> impl IntoView {
///     view! {
///         <Code color=Color::Accent variant=Variant::Subtle size=Size::Sm>
///             "console.log()"
///         </Code>
///     }
/// }
/// ```
#[component]
pub fn Code(
    /// The inline code content to display.
    children: Children,

    /// Color palette of the inline code element.
    #[prop(default = Color::Default)]
    color: Color,

    /// Visual style variant.
    #[prop(default = Variant::Subtle)]
    variant: Variant,

    /// Size of the inline code element.
    #[prop(default = Size::Sm)]
    size: Size,

    /// Additional CSS class names on the `<code>` element.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the `<code>` element.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the element.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label for screen readers.
    #[prop(default = "")]
    aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let color_style = match variant {
        Variant::Solid => color.to_solid_style(),
        Variant::Subtle => color.to_subtle_style(),
        Variant::Outline => color.to_outline_style(),
        Variant::Surface => color.to_surface_style(),
        Variant::Plain => "",
        Variant::Custom(_) => "",
    };

    let full_style = format!(
        "{} {} {} {}",
        base_code_style(),
        size.to_style(),
        color_style,
        style,
    );

    let element_class = format!(
        "code {} {} {} {}",
        variant.to_class(),
        color.to_class(),
        size.to_class(),
        class,
    );

    view! {
        <code
            id=id
            class=element_class
            style=full_style
            aria-label=aria_label
            data-testid=data_testid
        >
            {children()}
        </code>
    }
}

/// Shared context provided by [`Block`] to all child components.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct BlockContext {
    /// The raw code string stored in the code block.
    pub code: &'static str,
    /// The language of the code block.
    pub language: Language,
    /// The syntax-highlight theme.
    pub theme: Theme,
}

/// The header bar of a [`Block`], containing a title and action controls.
///
/// # Accessibility
///
/// - Renders as `<div>` with flex layout matching the dark header bar.
///
/// # Examples
///
/// ```rust
/// use coders::leptos::{Block, Content, Header, Title};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyHeader() -> impl IntoView {
///     view! {
///         <Block code="let x = 1;" language={coders::Language::Rust}>
///             <Header>
///                 <Title>"main.rs"</Title>
///             </Header>
///             <Content />
///         </Block>
///     }
/// }
/// ```
#[component]
pub fn Header(
    /// Slot content, title, badges, copy button.
    children: Children,

    /// Additional CSS class names on the header `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let theme = use_context::<BlockContext>()
        .map(|c| c.theme)
        .unwrap_or_default();
    view! {
        <div
            id=id
            class=format!("code-block__header {}", class)
            style=format!("{} {}", themed_header_style(theme), style)
        >
            {children()}
        </div>
    }
}

/// A title label inside a [`Header`].
#[component]
pub fn Title(
    /// Title text or node.
    children: Children,

    /// Additional CSS class names on the title `<span>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the title `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    view! {
        <span
            id=id
            class=format!("code-block__title {}", class)
            style=format!("{} {}", base_title_style(), style)
        >
            {children()}
        </span>
    }
}

/// A button that copies the [`Block`] code to the clipboard.
///
/// Reads [`BlockContext`] from a surrounding [`Block`].
///
/// # Accessibility
///
/// - Renders as `<button type="button">`.
/// - `aria-live="polite"` announces the copy confirmation to screen readers.
///
/// # Panics
///
/// Panics if used outside a [`Block`] context.
#[component]
pub fn CopyTrigger(
    /// Label shown before copying.
    #[prop(default = "Copy")]
    copy_label: &'static str,

    /// Label shown after a successful copy.
    #[prop(default = "Copied!")]
    copied_label: &'static str,

    /// Duration in milliseconds before label resets.
    #[prop(default = 2000)]
    reset_ms: u32,

    /// Additional CSS class names on the button.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the button.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label for screen readers.
    #[prop(default = "Copy code to clipboard")]
    aria_label: &'static str,
) -> impl IntoView {
    let ctx = use_context::<BlockContext>().expect("CopyTrigger must be inside Block");
    let copied = RwSignal::new(false);

    let on_click = move |_| {
        let code = ctx.code;
        spawner(async move {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(code)).await;
            }
            copied.set(true);
            TimeoutFuture::new(reset_ms).await;
            copied.set(false);
        });
    };

    let label = move || {
        if copied.get() {
            copied_label
        } else {
            copy_label
        }
    };
    let full_style = format!("{} {}", base_copy_trigger_style(), style);

    view! {
        <button
            id=id
            type="button"
            class=format!("code-block__copy-trigger {}", class)
            style=full_style
            on:click=on_click
            aria-label=aria_label
            aria-live="polite"
        >
            {label}
        </button>
    }
}

/// The scrollable code area inside a [`Block`].
///
/// When no children provided, renders the raw code from context in a `<pre>`.
///
/// # Panics
///
/// Panics if used outside a [`Block`] context.
#[component]
pub fn Content(
    /// Optional children rendered inside the content area.
    #[prop(optional)]
    children: Option<Children>,

    /// Additional CSS class names on the content `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,

    /// CSS class names on the inner `<pre>` element.
    #[prop(default = "")]
    pre_class: &'static str,

    /// Inline CSS on the inner `<pre>` element.
    #[prop(default = "")]
    pre_style: &'static str,
) -> impl IntoView {
    let ctx = use_context::<BlockContext>().expect("Content must be inside Block");

    let full_style = format!("{} {}", base_content_style(), style);
    let pre_full_style = format!("{} {}", themed_pre_style(ctx.theme), pre_style);
    let hl = crate::common::highlight_code(ctx.code, ctx.language, ctx.theme);

    view! {
        <div
            id=id
            class=format!("code-block__content {}", class)
            style=full_style
        >
            {match children {
                Some(ch) => view! { {ch()} }.into_any(),
                None => view! {
                    <pre
                        class=format!("code-block__pre {}", pre_class)
                        style=pre_full_style
                        tabindex="0"
                    >
                        <code
                            class=ctx.language.to_class()
                            inner_html=hl
                        />
                    </pre>
                }.into_any(),
            }}
        </div>
    }
}

/// A small language label in the [`Header`].
///
/// # Panics
///
/// Panics if used outside a [`Block`] context.
#[component]
pub fn LanguageBadge(
    /// Additional CSS class names.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS.
    #[prop(default = "")]
    style: &'static str,
) -> impl IntoView {
    let ctx = use_context::<BlockContext>().expect("LanguageBadge must be inside Block");

    view! {
        <span
            class=format!("code-block__lang {}", class)
            style=format!("{} {}", base_lang_badge_style(), style)
            aria-label=format!("Language: {}", ctx.language.to_label())
        >
            {ctx.language.to_label()}
        </span>
    }
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
/// use coders::leptos::{Block, Header, Title, CopyTrigger, Content};
/// use coders::Language;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyBlock() -> impl IntoView {
///     view! {
///         <Block code="fn main() {}" language={Language::Rust}>
///             <Header>
///                 <Title>"main.rs"</Title>
///                 <CopyTrigger />
///             </Header>
///             <Content />
///         </Block>
///     }
/// }
/// ```
#[component]
pub fn Block(
    /// The raw code string to display and copy.
    code: &'static str,

    /// The programming language for syntax highlighting and the language badge.
    #[prop(default = Language::Plain)]
    language: Language,

    /// Syntax-highlight colour theme (default: [`Theme::OneDark`]).
    #[prop(default = Theme::OneDark)]
    theme: Theme,

    /// Child components.
    #[prop(optional)]
    children: Option<Children>,

    /// Additional CSS class names on the root `<figure>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the root `<figure>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label for the region.
    #[prop(default = "Code block")]
    aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let ctx = BlockContext {
        code,
        language,
        theme,
    };
    provide_context(ctx);

    let full_style = format!("{} {}", themed_block_style(theme), style);
    let block_class = format!("code-block {} {}", language.to_class(), class);

    view! {
        <figure
            id=id
            class=block_class
            style=full_style
            role="region"
            aria-label=aria_label
            data-testid=data_testid
        >
            {match children {
                Some(ch) => view! { {ch()} }.into_any(),
                None => view! { <Content /> }.into_any(),
            }}
        </figure>
    }
}

fn spawner<F: std::future::Future<Output = ()> + 'static>(fut: F) {
    wasm_bindgen_futures::spawn_local(fut);
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
