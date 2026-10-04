// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use coders::yew::{Block, Code, Content, CopyTrigger, Header, LanguageBadge, Title};
use coders::{Color, Language, Size, Theme, Variant};
use yew::prelude::*;

#[function_component(ExampleBasicCode)]
pub fn example_basic_code() -> Html {
    html! { <Code>{ r#"console.log("Hello, world!")"# }</Code> }
}

#[function_component(ExampleSizes)]
pub fn example_sizes() -> Html {
    html! {
        <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
            <Code size={Size::Xs}>{ "console.log()" }</Code>
            <Code size={Size::Sm}>{ "console.log()" }</Code>
            <Code size={Size::Md}>{ "console.log()" }</Code>
            <Code size={Size::Lg}>{ "console.log()" }</Code>
        </div>
    }
}

#[function_component(ExampleVariants)]
pub fn example_variants() -> Html {
    html! {
        <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
            <Code variant={Variant::Solid}>{ "console.log()" }</Code>
            <Code variant={Variant::Outline}>{ "console.log()" }</Code>
            <Code variant={Variant::Subtle}>{ "console.log()" }</Code>
            <Code variant={Variant::Surface}>{ "console.log()" }</Code>
            <Code variant={Variant::Plain}>{ "console.log()" }</Code>
        </div>
    }
}

#[function_component(ExampleColors)]
pub fn example_colors() -> Html {
    let colors: Vec<(&'static str, Color)> = vec![
        ("Default", Color::Default),
        ("Accent", Color::Accent),
        ("Success", Color::Success),
        ("Warning", Color::Warning),
        ("Danger", Color::Danger),
        ("Teal", Color::Teal),
        ("Blue", Color::Blue),
        ("Pink", Color::Pink),
    ];
    html! {
        <div style="display: flex; flex-wrap: wrap; gap: 10px; align-items: flex-start;">
            { for colors.iter().map(|(name, color)| html! {
                <div style="display: flex; flex-direction: column; gap: 4px; align-items: center;">
                    <span style="font-size: 10px; color: #a0aec0;">{name}</span>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                        <Code color={*color} variant={Variant::Solid}>{"code"}</Code>
                        <Code color={*color} variant={Variant::Subtle}>{"code"}</Code>
                        <Code color={*color} variant={Variant::Outline}>{"code"}</Code>
                        <Code color={*color} variant={Variant::Surface}>{"code"}</Code>
                    </div>
                </div>
            }) }
        </div>
    }
}

#[function_component(ExampleBasicBlock)]
pub fn example_basic_block() -> Html {
    html! {
        <Block
            code=r#"fn main() {
    println!("Hello, world!");
}"#
            language={Language::Rust}
        />
    }
}

#[function_component(ExampleBlockWithHeader)]
pub fn example_block_with_header() -> Html {
    html! {
        <Block
            code=r#"fn main() {
    println!("Hello, world!");
}"#
            language={Language::Rust}
        >
            <Header>
                <Title>{ "main.rs" }</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockWithLangBadge)]
pub fn example_block_with_lang_badge() -> Html {
    html! {
        <Block
            code=r#"const greeting = "Hello, World!";
    console.log(greeting);"#
            language={Language::TypeScript}
        >
            <Header>
                <Title>{ "index.ts" }</Title>
                <div style="display: flex; align-items: center; gap: 8px;">
                    <LanguageBadge />
                    <CopyTrigger />
                </div>
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockBash)]
pub fn example_block_bash() -> Html {
    html! {
        <Block
            code="cargo add coders --features yew"
            language={Language::Bash}
            aria_label="Installation command"
        >
            <Header>
                <Title>{ "terminal" }</Title>
                <CopyTrigger copy_label="Copy" copied_label="✓ Copied" />
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockJson)]
pub fn example_block_json() -> Html {
    html! {
        <Block
            code=r#"{
  "name": "coders",
  "version": "0.1.0",
  "description": "A code component for WASM frameworks"
}"#
            language={Language::Json}
        >
            <Header>
                <Title>{ "package.json" }</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockPython)]
pub fn example_block_python() -> Html {
    html! {
        <Block
            code=r#"def greet(name: str) -> str:
    return f"Hello, {name}!"

print(greet("World"))"#
            language={Language::Python}
        >
            <Header>
                <div style="display: flex; align-items: center; gap: 8px;">
                    <LanguageBadge />
                    <Title>{ "greet.py" }</Title>
                </div>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockCustomStyle)]
pub fn example_block_custom_style() -> Html {
    html! {
        <Block
            code=r#"SELECT * FROM users WHERE active = true ORDER BY name;"#
            language={Language::Sql}
            style="border: 2px solid #7c3aed;"
        >
            <Header>
                <Title>{ "query.sql" }</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[function_component(ExampleBlockPlainText)]
pub fn example_block_plain_text() -> Html {
    html! {
        <Block code="npm install coders" language={Language::Bash} style="display: inline-flex;">
            <Content />
        </Block>
    }
}

#[function_component(ExampleThemes)]
pub fn example_themes() -> Html {
    let themes: &[(&str, Theme)] = &[
        ("OneDark", Theme::OneDark),
        ("Dracula", Theme::Dracula),
        ("NightOwl", Theme::NightOwl),
        ("GithubDark", Theme::GithubDark),
        ("GithubLight", Theme::GithubLight),
        ("Solarized", Theme::Solarized),
        ("Monokai", Theme::Monokai),
        ("Nord", Theme::Nord),
    ];
    let code = "fn add(a: u32, b: u32) -> u32 { a + b }";
    html! {
        <div style="display:flex;flex-direction:column;gap:6px;width:100%;">
            { for themes.iter().map(|(name, theme)| html! {
                <div style="display:flex;flex-direction:column;gap:2px;">
                    <span style="font-size:10px;color:#a0aec0;font-family:monospace;">{name}</span>
                    <Block
                        code={code}
                        language={Language::Rust}
                        theme={*theme}
                    >
                        <Content />
                    </Block>
                </div>
            }) }
        </div>
    }
}

#[function_component(ExampleInlineRealWorld)]
pub fn example_inline_real_world() -> Html {
    html! {
        <div
            style="display:flex;flex-direction:column;gap:16px;line-height:1.7;color:#000;font-size:14px;"
        >
            <p>
                { "Run " }
                <Code color={Color::Accent} variant={Variant::Subtle}>
                    { "cargo build --release" }
                </Code>
                { " to compile your project in release mode." }
            </p>
            <p>
                { "Set the " }
                <Code color={Color::Warning} variant={Variant::Surface}>{ "RUST_LOG=debug" }</Code>
                { " environment variable before running to enable verbose output." }
            </p>
            <p>
                { "The " }
                <Code color={Color::Success} variant={Variant::Outline}>
                    { "#[derive(Clone)]" }
                </Code>
                { " attribute auto-implements Clone for your struct." }
            </p>
            <p>
                { "Use " }
                <Code variant={Variant::Solid} color={Color::Danger}>{ "unsafe { ... }" }</Code>
                { " blocks only when you can guarantee memory safety yourself." }
            </p>
            <p>
                { "Prefer " }
                <Code size={Size::Sm} variant={Variant::Subtle} color={Color::Teal}>
                    { "Vec::with_capacity(n)" }
                </Code>
                { " over " }
                <Code size={Size::Sm} variant={Variant::Subtle} color={Color::Teal}>
                    { "Vec::new()" }
                </Code>
                { " when the length is known up-front." }
            </p>
        </div>
    }
}

#[function_component(ExampleCustomCode)]
pub fn example_custom_code() -> Html {
    html! {
        <div style="display:flex;flex-direction:column;gap:12px;">
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">
                    { "Color::Custom" }
                </span>
                <Code
                    color={Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;border-radius:4px;")}
                    variant={Variant::Plain}
                    size={Size::Md}
                >
                    { "gradient-badge" }
                </Code>
            </div>
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">
                    { "Size::Custom" }
                </span>
                <Code
                    size={Size::Custom("font-size:20px;padding:6px 14px;")}
                    color={Color::Accent}
                    variant={Variant::Solid}
                >
                    { "large custom snippet" }
                </Code>
            </div>
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">
                    { "Block Language::Custom" }
                </span>
                <Block
                    code="[rule]\ncolor = \"red\"\nweight = 600"
                    language={Language::Custom("toml-custom")}
                >
                    <Header>
                        <Title>{ "config.toml" }</Title>
                        <CopyTrigger />
                    </Header>
                    <Content />
                </Block>
            </div>
        </div>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Code RS Yew Examples" }</h1>
            <section aria-labelledby="inline-heading" class="w-full max-w-6xl mb-12">
                <h2 id="inline-heading" class="text-xl font-semibold text-white mb-6">
                    { "Inline Code" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Basic" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Code;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Code>
            {"console.log(\"Hello!\")"}
        </Code>
    }
}"# }
                        </pre>
                        <ExampleBasicCode />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Sizes" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Code;
use coders::Size;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <>
            <Code size={Size::Xs}>{"console.log()"}</Code>
            <Code size={Size::Sm}>{"console.log()"}</Code>
            <Code size={Size::Md}>{"console.log()"}</Code>
            <Code size={Size::Lg}>{"console.log()"}</Code>
        </>
    }
}"# }
                        </pre>
                        <ExampleSizes />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Variants" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Code;
use coders::Variant;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <>
            <Code variant={Variant::Solid}>{"console.log()"}</Code>
            <Code variant={Variant::Outline}>{"console.log()"}</Code>
            <Code variant={Variant::Subtle}>{"console.log()"}</Code>
            <Code variant={Variant::Surface}>{"console.log()"}</Code>
            <Code variant={Variant::Plain}>{"console.log()"}</Code>
        </>
    }
}"# }
                        </pre>
                        <ExampleVariants />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Colors" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Code;
use coders::{Color, Variant};
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <>
            <Code color={Color::Accent} variant={Variant::Solid}>
                {"code"}
            </Code>
            <Code color={Color::Success} variant={Variant::Subtle}>
                {"code"}
            </Code>
        </>
    }
}"# }
                        </pre>
                        <ExampleColors />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "All 8 Themes" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{Block, Content};
use coders::{Language, Theme};
use yew::prelude::*;

html! {
    <Block
        code="fn add(a: u32, b: u32) -> u32 { a + b }"
        language={Language::Rust}
        theme={Theme::Dracula}
    >
        <Content />
    </Block>
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleThemes />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Inline in Prose" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Code;
use coders::{Color, Size, Variant};
use yew::prelude::*;

html! {
    <p>
        {"Run "}<Code color={Color::Accent} variant={Variant::Subtle}>
            {"cargo build --release"}
        </Code>{" to compile."}
    </p>
    <p>
        {"Set "}<Code color={Color::Warning} variant={Variant::Surface}>
            {"RUST_LOG=debug"}
        </Code>{" for verbose output."}
    </p>
}"# }
                        </pre>
                        <ExampleInlineRealWorld />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Props" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{Code, Block, Content,
    Header, Title, CopyTrigger};
use coders::{Color, Language, Size, Variant};
use yew::prelude::*;

// Color::Custom, any inline CSS string
html! {
    <Code
        color={Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;")}
        variant={Variant::Plain}
        size={Size::Md}
    >
        {"gradient-badge"}
    </Code>
}

// Size::Custom, full font-size / padding override
html! {
    <Code
        size={Size::Custom("font-size:20px;padding:6px 14px;")}
        color={Color::Accent}
        variant={Variant::Solid}
    >
        {"large custom snippet"}
    </Code>
}

// Language::Custom, arbitrary language slug
html! {
    <Block
        code="[rule]\ncolor = \"red\""
        language={Language::Custom("toml-custom")}
    >
        <Header>
            <Title>{"config.toml"}</Title>
            <CopyTrigger />
        </Header>
        <Content />
    </Block>
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleCustomCode />
                        </div>
                    </article>
                </div>
            </section>
            <section aria-labelledby="block-heading " class="w-full max-w-6xl mb-12">
                <h2 id="block-heading " class="text-xl font-semibold text-white mb-6">
                    { "Code Block " }
                </h2>
                <div class="flex flex-col gap-8">
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Basic Block (Auto-render)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::Block;
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="fn main() { println!(\"Hello!\"); }"
            language={Language::Rust}
        />
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBasicBlock />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "With Header and Copy Button " }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{
    Block, Header, Title,
    CopyTrigger, Content,
};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="fn main() { println!(\"Hello!\"); }"
            language={Language::Rust}
        >
            <Header>
                <Title>{"main.rs"}</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockWithHeader />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "With Language Badge " }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="const greeting = \"Hello\";"
            language={Language::TypeScript}
        >
            <Header>
                <Title>{"index.ts"}</Title>
                <div style="display: flex; gap: 8px; align-items: center;">
                    <LanguageBadge />
                    <CopyTrigger />
                </div>
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockWithLangBadge />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Bash / Install Command " }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{
    Block, Header, Title,
    CopyTrigger, Content,
};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="cargo add coders --features yew"
            language={Language::Bash}
        >
            <Header>
                <Title>{"terminal"}</Title>
                <CopyTrigger
                    copy_label={"Copy"}
                    copied_label={"✓ Copied"}
                />
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockBash />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "JSON" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{Block, Header, Title, CopyTrigger, Content};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code=r#"{"name":"coders","version":"0.1.0"}"\#
            language={Language::Json}
        >
            <Header>
                <Title>{"package.json"}</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockJson />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Python" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="def greet(name: str) -> str:\n    return f'Hello, {name}!'"
            language={Language::Python}
        >
            <Header>
                <div style="display:flex;gap:8px;align-items:center;">
                    <LanguageBadge />
                    <Title>{"greet.py"}</Title>
                </div>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockPython />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Style Override " }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{Block, Header, Title, CopyTrigger, Content};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="SELECT * FROM users WHERE active = true;"
            language={Language::Sql}
            style={"border: 2px solid #7c3aed;"}
        >
            <Header>
                <Title>{"query.sql"}</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockCustomStyle />
                        </div>
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black "
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Plain Text (No Adapter)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre "
                        >
                            { r#"use coders::yew::{Block, Content};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block
            code="npm install coders"
            language={Language::Bash}
        >
            <Content />
        </Block>
    }
}"# }
                        </pre>
                        <div class="w-full ">
                            <ExampleBlockPlainText />
                        </div>
                    </article>
                </div>
            </section>
        </div>
    }
}
