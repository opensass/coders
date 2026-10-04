// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    Color, Language, Size, Theme, Variant, base_code_style, base_content_style,
    base_copy_trigger_style, base_lang_badge_style, base_title_style, themed_block_style,
    themed_header_style, themed_pre_style,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// Props for the [`Code`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct CodeProps {
    /// The inline code content to display.
    #[prop_or_default]
    pub children: Children,

    /// Color palette of the inline code element.
    #[prop_or_default]
    pub color: Color,

    /// Visual style variant: solid, subtle, outline, surface, or plain.
    #[prop_or_default]
    pub variant: Variant,

    /// Size of the inline code element.
    #[prop_or_default]
    pub size: Size,

    /// The underlying HTML element to render. Defaults to `"code"`.
    #[prop_or("code")]
    pub as_element: &'static str,

    /// Additional CSS class names on the `<code>` element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<code>` element.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the element.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label for screen readers.
    #[prop_or_default]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// An inline code snippet with configurable color, variant, and size.
///
/// Renders a `<code>` element (or a custom element via `as_element`) styled
/// with monospace font and the selected color/variant combination.
///
/// # Accessibility
///
/// - Renders as `<code>` by default, which provides implicit semantics for
///   screen readers indicating machine-readable content.
/// - An optional `aria_label` may be supplied for additional context.
///
/// # Examples
///
/// ```rust
/// use coders::yew::Code;
/// use coders::{Color, Size, Variant};
/// use yew::prelude::*;
///
/// #[function_component(InlineCode)]
/// pub fn inline_code() -> Html {
///     html! {
///         <Code color={Color::Accent} variant={Variant::Subtle} size={Size::Sm}>
///             {"console.log()"}
///         </Code>
///     }
/// }
/// ```
#[function_component(Code)]
pub fn code(props: &CodeProps) -> Html {
    let color_style = match props.variant {
        Variant::Solid => props.color.to_solid_style(),
        Variant::Subtle => props.color.to_subtle_style(),
        Variant::Outline => props.color.to_outline_style(),
        Variant::Surface => props.color.to_surface_style(),
        Variant::Plain => "",
        Variant::Custom(_) => "",
    };

    let full_style = format!(
        "{} {} {} {}",
        base_code_style(),
        props.size.to_style(),
        color_style,
        props.style,
    );

    let element_class = format!(
        "code {} {} {} {}",
        props.variant.to_class(),
        props.color.to_class(),
        props.size.to_class(),
        props.class,
    );

    html! {
        <code
            id={props.id}
            class={element_class}
            style={full_style}
            aria-label={props.aria_label}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </code>
    }
}

/// Props for the [`Header`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    /// Slot content, typically a [`Title`] and [`CopyTrigger`].
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the header `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the header element.
    #[prop_or_default]
    pub id: &'static str,
}

/// The header bar of a [`Block`], containing a title and action controls.
///
/// # Examples
///
/// ```rust
/// use coders::yew::{Block, Content, Header, Title};
/// use yew::prelude::*;
///
/// #[function_component(MyHeader)]
/// pub fn my_header() -> Html {
///     html! {
///         <Block code="let x = 1;" language={coders::Language::Rust}>
///             <Header>
///                 <Title>{"main.rs"}</Title>
///             </Header>
///             <Content />
///         </Block>
///     }
/// }
/// ```
#[function_component(Header)]
pub fn code_block_header(props: &HeaderProps) -> Html {
    let theme = use_context::<BlockContext>()
        .map(|c| c.theme)
        .unwrap_or_default();
    html! {
        <div
            id={props.id}
            class={format!("code-block__header {}", props.class)}
            style={format!("{} {}", themed_header_style(theme), props.style)}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for the [`Title`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct TitleProps {
    /// Title text or node (e.g. an icon + filename).
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the title `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the title `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute.
    #[prop_or_default]
    pub id: &'static str,
}

/// A title label inside a [`Header`].
#[function_component(Title)]
pub fn code_block_title(props: &TitleProps) -> Html {
    html! {
        <span
            id={props.id}
            class={format!("code-block__title {}", props.class)}
            style={format!("{} {}", base_title_style(), props.style)}
        >
            { for props.children.iter() }
        </span>
    }
}

/// Shared context provided by [`Block`] to all child components.
///
/// Provides the raw code string so that [`Content`] and
/// [`CopyTrigger`] can access it without prop drilling.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockContext {
    /// The raw code string stored in the code block.
    pub code: &'static str,
    /// The language of the code block.
    pub language: Language,
    /// The syntax-highlight theme.
    pub theme: Theme,
}

/// Props for the [`CopyTrigger`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct CopyTriggerProps {
    /// Label shown before copying.
    #[prop_or("Copy")]
    pub copy_label: &'static str,

    /// Label shown after a successful copy.
    #[prop_or("Copied!")]
    pub copied_label: &'static str,

    /// Duration in milliseconds before the label resets to `copy_label`.
    #[prop_or(2000)]
    pub reset_ms: u32,

    /// Additional CSS class names on the button.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the button.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the button.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label for the copy button.
    #[prop_or("Copy code to clipboard")]
    pub aria_label: &'static str,
}

/// A button that copies the [`Block`] code to the clipboard.
///
/// Must be used inside a [`Block`], reads [`BlockContext`] to
/// obtain the code string.
///
/// # Accessibility
///
/// - Renders as a `<button>` with type `"button"`.
/// - `aria-label` describes the action for screen readers.
/// - `aria-live="polite"` announces the copy confirmation.
///
/// # Panics
///
/// Panics in debug mode when used outside a [`Block`] context.
#[function_component(CopyTrigger)]
pub fn code_block_copy_trigger(props: &CopyTriggerProps) -> Html {
    let ctx = use_context::<BlockContext>().expect("CopyTrigger must be inside Block");
    let copied = use_state(|| false);

    let on_click = {
        let code = ctx.code;
        let copied = copied.clone();
        let reset_ms = props.reset_ms;
        Callback::from(move |_: MouseEvent| {
            let copied = copied.clone();
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let promise = clipboard.write_text(code);
            spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                copied.set(true);
                let copied2 = copied.clone();
                let closure = Closure::once(move || {
                    copied2.set(false);
                });
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        reset_ms as i32,
                    )
                    .ok();
                closure.forget();
            });
        })
    };

    let label = if *copied {
        props.copied_label
    } else {
        props.copy_label
    };

    let full_style = format!("{} {}", base_copy_trigger_style(), props.style);

    html! {
        <button
            id={props.id}
            type="button"
            class={format!("code-block__copy-trigger {}", props.class)}
            style={full_style}
            onclick={on_click}
            aria-label={props.aria_label}
            aria-live="polite"
        >
            {label}
        </button>
    }
}

/// Props for the [`Content`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct ContentProps {
    /// Optional children, rendered inside the scrollable content area.
    /// When omitted, the raw code from context is rendered inside a `<pre>`.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the content `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute.
    #[prop_or_default]
    pub id: &'static str,

    /// CSS class names on the inner `<pre>` element (when no children provided).
    #[prop_or_default]
    pub pre_class: &'static str,

    /// Inline CSS on the inner `<pre>` element (when no children provided).
    #[prop_or_default]
    pub pre_style: &'static str,
}

/// The scrollable code area inside a [`Block`].
///
/// When no children are provided, it reads [`BlockContext`] and renders
/// the raw code in a `<pre>` element automatically. Supply children to
/// render highlighted HTML or custom content instead.
///
/// # Panics
///
/// Panics in debug mode when used outside a [`Block`] context.
#[function_component(Content)]
pub fn code_block_content(props: &ContentProps) -> Html {
    let ctx = use_context::<BlockContext>().expect("Content must be inside Block");

    let has_children = props.children.iter().next().is_some();

    let full_style = format!("{} {}", base_content_style(), props.style);
    let pre_full_style = format!("{} {}", themed_pre_style(ctx.theme), props.pre_style);

    html! {
        <div
            id={props.id}
            class={format!("code-block__content {}", props.class)}
            style={full_style}
        >
            if has_children {
                { for props.children.iter() }
            } else {
                <pre
                    class={format!("code-block__pre {}", props.pre_class)}
                    style={pre_full_style}
                    tabindex={"0"}
                >
                    { Html::from_html_unchecked(
                        format!(
                            "<code class=\"{}\">{}</code>",
                            ctx.language.to_class(),
                            crate::common::highlight_code(ctx.code, ctx.language, ctx.theme)
                        ).into()
                    ) }
                </pre>
            }
        </div>
    }
}

/// Props for the [`LanguageBadge`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct LanguageBadgeProps {
    /// Additional CSS class names on the badge `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the badge `<span>`.
    #[prop_or_default]
    pub style: &'static str,
}

/// A small language label rendered in the [`Header`].
///
/// Reads [`BlockContext`] to obtain the language. Must be inside [`Block`].
#[function_component(LanguageBadge)]
pub fn code_block_language_badge(props: &LanguageBadgeProps) -> Html {
    let ctx = use_context::<BlockContext>().expect("LanguageBadge must be inside Block");

    html! {
        <span
            class={format!("code-block__lang {}", props.class)}
            style={format!("{} {}", base_lang_badge_style(), props.style)}
            aria-label={format!("Language: {}", ctx.language.to_label())}
        >
            {ctx.language.to_label()}
        </span>
    }
}

/// Props for the [`Block`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct BlockProps {
    /// The raw code string to display and copy.
    pub code: &'static str,

    /// The programming language for syntax highlighting and the language badge.
    #[prop_or_default]
    pub language: Language,

    /// Syntax-highlight colour theme (default: [`Theme::OneDark`]).
    #[prop_or_default]
    pub theme: Theme,

    /// Child components: [`Header`], [`Content`], etc.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the root `<figure>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the root `<figure>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the root element.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label for the code block region.
    #[prop_or("Code block")]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A composable code-block container providing context and structure.
///
/// Provides [`BlockContext`] with the `code` and `language` props to all
/// descendant components. When no children are provided, a `<Content />`
/// is rendered automatically showing the raw code in a `<pre>` element.
///
/// # Accessibility
///
/// - Renders as `<figure role="region">` with `aria-label` for landmark navigation.
/// - Inner `<pre>` uses `tabindex="0"` so keyboard users can scroll it.
///
/// # Examples
///
/// ```rust
/// use coders::yew::{Block, Header, Title, CopyTrigger, Content};
/// use coders::Language;
/// use yew::prelude::*;
///
/// #[function_component(MyBlock)]
/// pub fn my_code_block() -> Html {
///     html! {
///         <Block code="fn main() { println!(\"Hello\"); }" language={Language::Rust}>
///             <Header>
///                 <Title>{"main.rs"}</Title>
///                 <CopyTrigger />
///             </Header>
///             <Content />
///         </Block>
///     }
/// }
/// ```
#[function_component(Block)]
pub fn code_block(props: &BlockProps) -> Html {
    let ctx = BlockContext {
        code: props.code,
        language: props.language,
        theme: props.theme,
    };

    let has_children = props.children.iter().next().is_some();
    let full_style = format!("{} {}", themed_block_style(props.theme), props.style);
    let block_class = format!("code-block {} {}", props.language.to_class(), props.class);

    html! {
        <ContextProvider<BlockContext> context={ctx}>
            <figure
                id={props.id}
                class={block_class}
                style={full_style}
                role="region"
                aria-label={props.aria_label}
                data-testid={props.data_testid}
            >
                if has_children {
                    { for props.children.iter() }
                } else {
                    <Content />
                }
            </figure>
        </ContextProvider<BlockContext>>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
