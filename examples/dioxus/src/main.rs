// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use coders::dioxus::{Block, Code, Content, CopyTrigger, Header, LanguageBadge, Title};
use coders::{Color, Language, Size, Theme, Variant};
use dioxus::prelude::*;

const RUST_HELLO: &str = "fn main() {\n    println!(\"Hello, world!\");\n}";
const TS_CONST: &str = "const greeting = \"Hello, World!\";\nconsole.log(greeting);";
const JSON_SAMPLE: &str = "{\n  \"name\": \"coders\",\n  \"version\": \"0.1.0\"\n}";
const PYTHON_GREET: &str =
    "def greet(name):\n    return f'Hello, {0}!'.format(name)\n\nprint(greet('World'))";

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        LandingPage {}
    }
}

#[component]
fn ExampleBasicCode() -> Element {
    rsx! {
        Code { r#"console.log("Hello, world!")"# }
    }
}

#[component]
fn ExampleSizes() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
            Code { size: Size::Xs, "console.log()" }
            Code { size: Size::Sm, "console.log()" }
            Code { size: Size::Md, "console.log()" }
            Code { size: Size::Lg, "console.log()" }
        }
    }
}

#[component]
fn ExampleVariants() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
            Code { variant: Variant::Solid,   "console.log()" }
            Code { variant: Variant::Outline,  "console.log()" }
            Code { variant: Variant::Subtle,   "console.log()" }
            Code { variant: Variant::Surface,  "console.log()" }
            Code { variant: Variant::Plain,    "console.log()" }
        }
    }
}

#[component]
fn ExampleColors() -> Element {
    let colors = [
        ("Default", Color::Default),
        ("Accent", Color::Accent),
        ("Success", Color::Success),
        ("Warning", Color::Warning),
        ("Danger", Color::Danger),
        ("Teal", Color::Teal),
        ("Blue", Color::Blue),
        ("Pink", Color::Pink),
    ];
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: flex-start;",
            for (name, color) in colors.iter() {
                div { style: "display: flex; flex-direction: column; gap: 4px; align-items: center;",
                    span { style: "font-size: 10px; color: #a0aec0;", "{name}" }
                    div { style: "display: flex; flex-direction: column; gap: 4px;",
                        Code { color: *color, variant: Variant::Solid,   "code" }
                        Code { color: *color, variant: Variant::Subtle,  "code" }
                        Code { color: *color, variant: Variant::Outline, "code" }
                        Code { color: *color, variant: Variant::Surface, "code" }
                    }
                }
            }
        }
    }
}

#[component]
fn ExampleBasicBlock() -> Element {
    rsx! {
        Block {
            code: RUST_HELLO,
            language: Language::Rust,
        }
    }
}

#[component]
fn ExampleBlockWithHeader() -> Element {
    rsx! {
        Block {
            code: RUST_HELLO,
            language: Language::Rust,
            Header {
                Title { "main.rs" }
                CopyTrigger {}
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockWithLangBadge() -> Element {
    rsx! {
        Block {
            code: TS_CONST,
            language: Language::TypeScript,
            Header {
                Title { "index.ts" }
                div { style: "display: flex; align-items: center; gap: 8px;",
                    LanguageBadge {}
                    CopyTrigger {}
                }
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockBash() -> Element {
    rsx! {
        Block {
            code: "cargo add coders --features dio",
            language: Language::Bash,
            aria_label: "Installation command",
            Header {
                Title { "terminal" }
                CopyTrigger { copy_label: "Copy", copied_label: "✓ Copied" }
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockJson() -> Element {
    rsx! {
        Block {
            code: JSON_SAMPLE,
            language: Language::Json,
            Header {
                Title { "package.json" }
                CopyTrigger {}
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockPython() -> Element {
    rsx! {
        Block {
            code: PYTHON_GREET,
            language: Language::Python,
            Header {
                div { style: "display: flex; align-items: center; gap: 8px;",
                    LanguageBadge {}
                    Title { "greet.py" }
                }
                CopyTrigger {}
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockCustomStyle() -> Element {
    rsx! {
        Block {
            code: "SELECT * FROM users WHERE active = true ORDER BY name;",
            language: Language::Sql,
            style: "border: 2px solid #7c3aed;",
            Header {
                Title { "query.sql" }
                CopyTrigger {}
            }
            Content {}
        }
    }
}

#[component]
fn ExampleBlockPlainText() -> Element {
    rsx! {
        Block {
            code: "npm install coders",
            language: Language::Bash,
            Content {}
        }
    }
}

const RUST_ADD: &str = "fn add(a: u32, b: u32) -> u32 { a + b }";

#[component]
fn ExampleThemes() -> Element {
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
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:6px;width:100%;",
            for (name, theme) in themes.iter() {
                div { style: "display:flex;flex-direction:column;gap:2px;",
                    span { style: "font-size:10px;color:#a0aec0;font-family:monospace;", "{name}" }
                    Block {
                        code: RUST_ADD,
                        language: Language::Rust,
                        theme: *theme,
                        Content {}
                    }
                }
            }
        }
    }
}

#[component]
fn ExampleInlineRealWorld() -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:16px;line-height:1.7;color:#000;font-size:14px;",
            p {
                "Run "
                Code { color: Color::Accent, variant: Variant::Subtle, "cargo build --release" }
                " to compile your project in release mode."
            }
            p {
                "Set the "
                Code { color: Color::Warning, variant: Variant::Surface, "RUST_LOG=debug" }
                " environment variable for verbose output."
            }
            p {
                "The "
                Code { color: Color::Success, variant: Variant::Outline, "#[derive(Clone)]" }
                " attribute auto-implements Clone for your struct."
            }
            p {
                "Use "
                Code { color: Color::Danger, variant: Variant::Solid, "unsafe {{ ... }}" }
                " only when you can guarantee memory safety."
            }
            p {
                "Prefer "
                Code { color: Color::Teal, variant: Variant::Subtle, size: Size::Sm, "Vec::with_capacity(n)" }
                " over "
                Code { color: Color::Teal, variant: Variant::Subtle, size: Size::Sm, "Vec::new()" }
                " when length is known up-front."
            }
        }
    }
}

const TOML_RULE: &str = "[rule]\ncolor = \"red\"\nweight = 600";

#[component]
fn ExampleCustomCode() -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:12px;",
            div {
                span { style: "font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;", "Color::Custom" }
                Code {
                    color: Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;border-radius:4px;"),
                    variant: Variant::Plain,
                    size: Size::Md,
                    "gradient-badge"
                }
            }
            div {
                span { style: "font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;", "Size::Custom" }
                Code {
                    size: Size::Custom("font-size:20px;padding:6px 14px;"),
                    color: Color::Accent,
                    variant: Variant::Solid,
                    "large custom snippet"
                }
            }
            div {
                span { style: "font-size:10px;color:#a0aec0;display:block;margin-bottom:4px;", "Language::Custom" }
                Block {
                    code: TOML_RULE,
                    language: Language::Custom("toml-custom"),
                    Header {
                        Title { "config.toml" }
                        CopyTrigger {}
                    }
                    Content {}
                }
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center",
                    style: "color: #5e5c7f; background-color: #303030; font-family: 'Rubik', sans-serif; overflow-x: hidden;",

            h1 { class: "text-3xl font-bold mb-8 text-white", "Code RS Dioxus Examples" }

            section { aria_labelledby: "inline-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "inline-heading", class: "text-xl font-semibold text-white mb-6", "Inline Code" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Basic" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Code;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Code {{ "console.log(\"Hello!\")" }}
    }}
}}"# }
                        ExampleBasicCode {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Sizes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Code;
use coders::Size;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Code {{ size: Size::Xs, "console.log()" }}
        Code {{ size: Size::Sm, "console.log()" }}
        Code {{ size: Size::Md, "console.log()" }}
        Code {{ size: Size::Lg, "console.log()" }}
    }}
}}"# }
                        ExampleSizes {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Variants" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Code;
use coders::Variant;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Code {{ variant: Variant::Solid,   "console.log()" }}
        Code {{ variant: Variant::Outline, "console.log()" }}
        Code {{ variant: Variant::Subtle,  "console.log()" }}
        Code {{ variant: Variant::Surface, "console.log()" }}
        Code {{ variant: Variant::Plain,   "console.log()" }}
    }}
}}"# }
                        ExampleVariants {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Colors" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Code;
use coders::{{Color, Variant}};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Code {{ color: Color::Accent, variant: Variant::Solid, "code" }}
        Code {{ color: Color::Success, variant: Variant::Subtle, "code" }}
        Code {{ color: Color::Danger, variant: Variant::Outline, "code" }}
    }}
}}"# }
                        ExampleColors {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "All 8 Themes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{Block, Content}};
use coders::{{Language, Theme}};
use dioxus::prelude::*;

const RUST_ADD: &str = "fn add(a: u32, b: u32) -> u32 {{ a + b }}";

// theme: Theme::OneDark | Dracula | NightOwl | GithubDark
// theme: Theme::GithubLight | Solarized | Monokai | Nord
#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: RUST_ADD,
            language: Language::Rust,
            theme: Theme::Dracula,
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width:100%;", ExampleThemes {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Inline in Prose" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Code;
use coders::{{Color, Size, Variant}};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        p {{
            "Run "
            Code {{ color: Color::Accent, variant: Variant::Subtle,
                "cargo build --release" }}
            " to compile in release mode."
        }}
        p {{
            "Set "
            Code {{ color: Color::Warning, variant: Variant::Surface,
                "RUST_LOG=debug" }}
            " for verbose output."
        }}
        p {{
            "Use "
            Code {{ color: Color::Danger, variant: Variant::Solid,
                "unsafe {{ ... }}" }}
            " only when memory safety is guaranteed."
        }}
    }}
}}"# }
                        ExampleInlineRealWorld {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Custom Props" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{Code, Block, Content,
    Header, Title, CopyTrigger}};
use coders::{{Color, Language, Size, Variant}};
use dioxus::prelude::*;

// Color::Custom, any inline CSS value string
Code {{ color: Color::Custom("background:linear-gradient(90deg,#f97316,#ec4899);color:#fff;"),
    variant: Variant::Plain, size: Size::Md, "gradient-badge" }}

// Size::Custom, full font-size / padding override
Code {{ size: Size::Custom("font-size:20px;padding:6px 14px;"),
    color: Color::Accent, variant: Variant::Solid, "large custom snippet" }}

// Language::Custom, arbitrary language slug
const TOML: &str = "[rule]\ncolor = \"red\"";
Block {{ code: TOML, language: Language::Custom("toml-custom"),
    Header {{
        Title {{ "config.toml" }}
        CopyTrigger {{}}
    }}
    Content {{}}
}}"# }
                        div { style: "width:100%;", ExampleCustomCode {} }
                    }
                }
            }

            section { aria_labelledby: "block-heading ", class: "w-full max-w-6xl mb-12",
                h2 { id: "block-heading ", class: "text-xl font-semibold text-white mb-6", "Code Block " }
                div { class: "flex flex-col gap-8",

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Basic Block (Auto-render)" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::Block;
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "fn main() {{ println!(\"Hello!\"); }}",
            language: Language::Rust,
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBasicBlock {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "With Header and Copy Button " }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "fn main() {{ println!(\"Hello!\"); }}",
            language: Language::Rust,
            Header {{
                Title {{ "main.rs" }}
                CopyTrigger {{}}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockWithHeader {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "With Language Badge " }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "const x = 1;",
            language: Language::TypeScript,
            Header {{
                Title {{ "index.ts" }}
                div {{ style: "display:flex;gap:8px;",
                    LanguageBadge {{}}
                    CopyTrigger {{}}
                }}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockWithLangBadge {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Bash Install Command " }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "cargo add coders --features dio",
            language: Language::Bash,
            Header {{
                Title {{ "terminal" }}
                CopyTrigger {{
                    copy_label: "Copy",
                    copied_label: "✓ Copied",
                }}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockBash {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "JSON" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "{{\n  \"name\": \"coders\",\n  \"version\": \"0.1.0\"\n}}",
            language: Language::Json,
            Header {{
                Title {{ "package.json" }}
                CopyTrigger {{}}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockJson {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Python" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
    LanguageBadge,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "def greet(name: str) -> str:\n    return f'Hello, {{name}}!'",
            language: Language::Python,
            Header {{
                div {{ style: "display:flex;gap:8px;",
                    LanguageBadge {{}}
                    Title {{ "greet.py" }}
                }}
                CopyTrigger {{}}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockPython {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Custom Style " }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{
    Block, Header, Title,
    CopyTrigger, Content,
}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "SELECT * FROM users;",
            language: Language::Sql,
            style: "border: 2px solid #7c3aed;",
            Header {{
                Title {{ "query.sql" }}
                CopyTrigger {{}}
            }}
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockCustomStyle {} }
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black ",
                        h3 { class: "text-xl font-bold mb-2", "Plain Text (No Adapter) " }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre ",
r#"use coders::dioxus::{{Block, Content}};
use coders::Language;
use dioxus::prelude::*;

#[component]
fn Example() -> Element {{
    rsx! {{
        Block {{
            code: "npm install coders",
            language: Language::Bash,
            Content {{}}
        }}
    }}
}}"# }
                        div { style: "width: 100%;", ExampleBlockPlainText {} }
                    }
                }
            }
        }
    }
}
