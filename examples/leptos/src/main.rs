// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use coders::leptos::{Block, Code, Content, CopyTrigger, Header, LanguageBadge, Title};
use coders::{Color, Language, Size, Theme, Variant};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(LandingPage);
}

#[component]
fn ExampleBasicCode() -> impl IntoView {
    view! {
        <Code>{r#"console.log("Hello, world!")"#}</Code>
    }
}

#[component]
fn ExampleSizes() -> impl IntoView {
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
            <Code size=Size::Xs>"console.log()"</Code>
            <Code size=Size::Sm>"console.log()"</Code>
            <Code size=Size::Md>"console.log()"</Code>
            <Code size=Size::Lg>"console.log()"</Code>
        </div>
    }
}

#[component]
fn ExampleVariants() -> impl IntoView {
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
            <Code variant=Variant::Solid>  "console.log()"</Code>
            <Code variant=Variant::Outline>"console.log()"</Code>
            <Code variant=Variant::Subtle> "console.log()"</Code>
            <Code variant=Variant::Surface>"console.log()"</Code>
            <Code variant=Variant::Plain>  "console.log()"</Code>
        </div>
    }
}

#[component]
fn ExampleColors() -> impl IntoView {
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
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 10px; align-items: flex-start;">
            { colors.into_iter().map(|(name, color)| view! {
                <div style="display: flex; flex-direction: column; gap: 4px; align-items: center;">
                    <span style="font-size: 10px; color: #a0aec0;">{name}</span>
                    <div style="display: flex; flex-direction: column; gap: 4px;">
                        <Code color=color variant=Variant::Solid>  "code"</Code>
                        <Code color=color variant=Variant::Subtle> "code"</Code>
                        <Code color=color variant=Variant::Outline>"code"</Code>
                        <Code color=color variant=Variant::Surface>"code"</Code>
                    </div>
                </div>
            }).collect_view() }
        </div>
    }
}

#[component]
fn ExampleBasicBlock() -> impl IntoView {
    view! {
        <Block
            code="fn main() {\n    println!(\"Hello, world!\");\n}"
            language=Language::Rust
        />
    }
}

#[component]
fn ExampleBlockWithHeader() -> impl IntoView {
    view! {
        <Block
            code="fn main() {\n    println!(\"Hello, world!\");\n}"
            language=Language::Rust
        >
            <Header>
                <Title>"main.rs"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockWithLangBadge() -> impl IntoView {
    view! {
        <Block
            code="const greeting = \"Hello, World!\";\nconsole.log(greeting);"
            language=Language::TypeScript
        >
            <Header>
                <Title>"index.ts"</Title>
                <div style="display: flex; align-items: center; gap: 8px;">
                    <LanguageBadge />
                    <CopyTrigger />
                </div>
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockBash() -> impl IntoView {
    view! {
        <Block
            code="cargo add coders --features lep"
            language=Language::Bash
            aria_label="Installation command"
        >
            <Header>
                <Title>"terminal"</Title>
                <CopyTrigger copy_label="Copy" copied_label="✓ Copied" />
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockJson() -> impl IntoView {
    view! {
        <Block
            code="{\n  \"name\": \"coders\",\n  \"version\": \"0.1.0\"\n}"
            language=Language::Json
        >
            <Header>
                <Title>"package.json"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockPython() -> impl IntoView {
    view! {
        <Block
            code="def greet(name: str) -> str:\n    return f\"Hello, {name}!\"\n\nprint(greet(\"World\"))"
            language=Language::Python
        >
            <Header>
                <div style="display: flex; align-items: center; gap: 8px;">
                    <LanguageBadge />
                    <Title>"greet.py"</Title>
                </div>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockCustomStyle() -> impl IntoView {
    view! {
        <Block
            code="SELECT * FROM users WHERE active = true ORDER BY name;"
            language=Language::Sql
            style="border: 2px solid #7c3aed;"
        >
            <Header>
                <Title>"query.sql"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}

#[component]
fn ExampleBlockPlainText() -> impl IntoView {
    view! {
        <Block
            code="npm install coders"
            language=Language::Bash
        >
            <Content />
        </Block>
    }
}

#[component]
fn ExampleThemes() -> impl IntoView {
    let code = "fn add(a: u32, b: u32) -> u32 { a + b }";
    let themes = [
        ("OneDark", Theme::OneDark),
        ("Dracula", Theme::Dracula),
        ("NightOwl", Theme::NightOwl),
        ("GithubDark", Theme::GithubDark),
        ("GithubLight", Theme::GithubLight),
        ("Solarized", Theme::Solarized),
        ("Monokai", Theme::Monokai),
        ("Nord", Theme::Nord),
    ];
    view! {
        <div style="display:flex;flex-direction:column;gap:6px;width:100%;">
            {themes.into_iter().map(|(name, theme)| view! {
                <div style="display:flex;flex-direction:column;gap:2px;">
                    <span style="font-size:10px;color:#a0aec0;font-family:monospace;">{name}</span>
                    <Block code=code language=Language::Rust theme=theme>
                        <Content />
                    </Block>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn ExampleInlineRealWorld() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:16px;line-height:1.7;color:#000;font-size:14px;">
            <p>
                "Run "
                <Code color=Color::Accent variant=Variant::Subtle>"cargo build --release"</Code>
                " to compile your project in release mode."
            </p>
            <p>
                "Set the "
                <Code color=Color::Warning variant=Variant::Surface>"RUST_LOG=debug"</Code>
                " environment variable for verbose output."
            </p>
            <p>
                "The "
                <Code color=Color::Success variant=Variant::Outline>"#[derive(Clone)]"</Code>
                " attribute auto-implements Clone for your struct."
            </p>
            <p>
                "Use "
                <Code variant=Variant::Solid color=Color::Danger>"unsafe { ... }"</Code>
                " only when you can guarantee memory safety yourself."
            </p>
            <p>
                "Prefer "
                <Code size=Size::Sm variant=Variant::Subtle color=Color::Teal>"Vec::with_capacity(n)"</Code>
                " over "
                <Code size=Size::Sm variant=Variant::Subtle color=Color::Teal>"Vec::new()"</Code>
                " when the length is known up-front."
            </p>
        </div>
    }
}

#[component]
fn ExampleCustomCode() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;">
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">"Color::Custom"</span>
                <Code
                    color=Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;border-radius:4px;")
                    variant=Variant::Plain
                    size=Size::Md
                >
                    "gradient-badge"
                </Code>
            </div>
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">"Size::Custom"</span>
                <Code size=Size::Custom("font-size:20px;padding:6px 14px;") color=Color::Accent variant=Variant::Solid>
                    "large custom snippet"
                </Code>
            </div>
            <div>
                <span style="font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;">"Language::Custom"</span>
                <Block
                    code="[rule]\ncolor = \"red\"\nweight = 600"
                    language=Language::Custom("toml-custom")
                >
                    <Header>
                        <Title>"config.toml"</Title>
                        <CopyTrigger />
                    </Header>
                    <Content />
                </Block>
            </div>
        </div>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">"Code RS Leptos Examples"</h1>

            <section aria-labelledby="inline-heading" class="w-full max-w-6xl mb-12">
                <h2 id="inline-heading" class="text-xl font-semibold text-white mb-6">"Inline Code"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Basic"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Code;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Code>
            {"console.log(\"Hello!\")"}
        </Code>
    }
}"#}</pre>
                        <ExampleBasicCode />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Sizes"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Code;
use coders::Size;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Code size=Size::Xs>"console.log()"</Code>
        <Code size=Size::Sm>"console.log()"</Code>
        <Code size=Size::Md>"console.log()"</Code>
        <Code size=Size::Lg>"console.log()"</Code>
    }
}"#}</pre>
                        <ExampleSizes />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Variants"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Code;
use coders::Variant;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Code variant=Variant::Solid>  "console.log()"</Code>
        <Code variant=Variant::Outline>"console.log()"</Code>
        <Code variant=Variant::Subtle> "console.log()"</Code>
        <Code variant=Variant::Surface>"console.log()"</Code>
        <Code variant=Variant::Plain>  "console.log()"</Code>
    }
}"#}</pre>
                        <ExampleVariants />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Colors"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Code;
use coders::{Color, Variant};
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Code color=Color::Accent  variant=Variant::Solid>  "code"</Code>
        <Code color=Color::Success variant=Variant::Subtle> "code"</Code>
        <Code color=Color::Danger  variant=Variant::Outline>"code"</Code>
    }
}"#}</pre>
                        <ExampleColors />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"All 8 Themes"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{Block, Content};
use coders::{Language, Theme};
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        // theme=Theme::OneDark | Dracula | NightOwl | GithubDark
        // theme=Theme::GithubLight | Solarized | Monokai | Nord
        <Block
            code="fn add(a: u32, b: u32) -> u32 { a + b }"
            language=Language::Rust
            theme=Theme::Dracula
        >
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width:100%;"><ExampleThemes /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Inline in Prose"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Code;
use coders::{Color, Size, Variant};
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <p>
            "Run "
            <Code color=Color::Accent variant=Variant::Subtle>
                "cargo build --release"
            </Code>
            " to compile in release mode."
        </p>
        <p>
            "Set "
            <Code color=Color::Warning variant=Variant::Surface>
                "RUST_LOG=debug"
            </Code>
            " for verbose output."
        </p>
        <p>
            "Use "
            <Code color=Color::Danger variant=Variant::Solid>
                "unsafe { ... }"
            </Code>
            " only when memory safety is guaranteed."
        </p>
    }
}"#}</pre>
                        <ExampleInlineRealWorld />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Custom Props"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{Code, Block, Content,
    Header, Title, CopyTrigger};
use coders::{Color, Language, Size, Variant};
use leptos::prelude::*;

// Color::Custom - any inline CSS value string
view! {
    <Code
        color=Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;")
        variant=Variant::Plain size=Size::Md
    >
        "gradient-badge"
    </Code>
}

// Size::Custom - full font-size / padding override
view! {
    <Code
        size=Size::Custom("font-size:20px;padding:6px 14px;")
        color=Color::Accent variant=Variant::Solid
    >
        "large custom snippet"
    </Code>
}

// Language::Custom - arbitrary language slug
view! {
    <Block
        code="[rule]\ncolor = \"red\""
        language=Language::Custom("toml-custom")
    >
        <Header>
            <Title>"config.toml"</Title>
            <CopyTrigger />
        </Header>
        <Content />
    </Block>
}"#}</pre>
                        <div style="width:100%;"><ExampleCustomCode /></div>
                    </article>

                </div>
            </section>

            <section aria-labelledby="block-heading " class="w-full max-w-6xl mb-12">
                <h2 id="block-heading " class="text-xl font-semibold text-white mb-6">"Code Block "</h2>
                <div class="flex flex-col gap-8">

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Basic Block (Auto-render)"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::Block;
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="fn main() { println!(\"Hello!\"); }"
            language=Language::Rust
        />
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBasicBlock /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"With Header and Copy Button "</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{
    Block, Header, Title,
    CopyTrigger, Content,
};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="fn main() { println!(\"Hello!\"); }"
            language=Language::Rust
        >
            <Header>
                <Title>"main.rs"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockWithHeader /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"With Language Badge "</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="const x = 1;"
            language=Language::TypeScript
        >
            <Header>
                <Title>"index.ts"</Title>
                <div style="display:flex;gap:8px;">
                    <LanguageBadge />
                    <CopyTrigger />
                </div>
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockWithLangBadge /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Bash Install Command "</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{
    Block, Header, Title,
    CopyTrigger, Content,
};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="cargo add coders --features lep"
            language=Language::Bash
        >
            <Header>
                <Title>"terminal"</Title>
                <CopyTrigger
                    copy_label="Copy"
                    copied_label="✓ Copied"
                />
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockBash /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"JSON"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{Block, Header, Title, CopyTrigger, Content};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="{\n  \"name\": \"coders\",\n  \"version\": \"0.1.0\"\n}"
            language=Language::Json
        >
            <Header>
                <Title>"package.json"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockJson /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Python"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="def greet(name: str) -> str:\n    return f'Hello, {name}!'"
            language=Language::Python
        >
            <Header>
                <div style="display:flex;gap:8px;">
                    <LanguageBadge />
                    <Title>"greet.py"</Title>
                </div>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockPython /></div>
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ">
                        <h3 class="text-xl font-bold mb-2">"Custom Style "</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ">{
r#"use coders::leptos::{Block, Header, Title, CopyTrigger, Content};
use coders::Language;
use leptos::prelude::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Block
            code="SELECT * FROM users WHERE active = true;"
            language=Language::Sql
            style="border: 2px solid #7c3aed;"
        >
            <Header>
                <Title>"query.sql"</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}"#}</pre>
                        <div style="width: 100%;"><ExampleBlockCustomStyle /></div>
                    </article>

                </div>
            </section>
        </div>
    }
}
