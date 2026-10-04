// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Controls the rendered size of a [`Code`] or [`Block`] component.
///
/// Each variant maps to specific font-size and padding CSS values. Use
/// [`Size::Custom`] for arbitrary inline-CSS overrides.
///
/// # Default
///
/// [`Size::Sm`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use coders::Size;
///
/// let style = Size::Md.to_style();
/// assert!(style.contains("14px"));
///
/// let custom = Size::Custom("font-size: 16px; padding: 2px 6px;");
/// assert!(custom.to_style().contains("16px"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Size {
    /// Extra-small: 10px font, 1px 3px padding.
    Xs,

    /// Small: 12px font, 2px 5px padding. This is the default.
    #[default]
    Sm,

    /// Medium: 14px font, 2px 6px padding.
    Md,

    /// Large: 16px font, 3px 8px padding.
    Lg,

    /// Arbitrary inline CSS, e.g. `"font-size: 13px; padding: 2px 5px;"`.
    Custom(&'static str),
}

impl Size {
    /// Returns the inline CSS `font-size` and `padding` string for this size variant.
    ///
    /// # Returns
    ///
    /// A `&'static str` with `font-size` and `padding` set for the named size,
    /// or the custom CSS string for [`Size::Custom`].
    pub fn to_style(self) -> &'static str {
        match self {
            Self::Xs => "font-size: 10px; padding: 1px 3px;",
            Self::Sm => "font-size: 12px; padding: 2px 5px;",
            Self::Md => "font-size: 14px; padding: 2px 6px;",
            Self::Lg => "font-size: 16px; padding: 3px 8px;",
            Self::Custom(s) => s,
        }
    }

    /// Returns the BEM modifier CSS class for this size.
    ///
    /// # Returns
    ///
    /// One of `"code--xs"`, `"code--sm"`, `"code--md"`, `"code--lg"`, or `"code--custom"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Xs => "code--xs",
            Self::Sm => "code--sm",
            Self::Md => "code--md",
            Self::Lg => "code--lg",
            Self::Custom(_) => "code--custom",
        }
    }
}

/// Color palette applied to the [`Code`] component.
///
/// Each variant maps to a distinct set of background, text, and border
/// colors across all visual [`Variant`]s.
///
/// # Default
///
/// [`Color::Default`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use coders::Color;
///
/// let style = Color::Accent.to_solid_style();
/// assert!(style.contains("#7c3aed"));
///
/// let cls = Color::Danger.to_class();
/// assert_eq!(cls, "code--danger");
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Color {
    /// Neutral gray.
    #[default]
    Default,
    Red,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Cyan,
    /// Purple accent: `#7c3aed`.
    Accent,
    Pink,
    /// Success green: `#16a34a`.
    Success,
    /// Warning amber: `#d97706`.
    Warning,
    /// Danger red: `#dc2626`.
    Danger,
    /// Fully custom colour.  Supply the full inline CSS for every variant style
    /// as a single string, e.g. `"background-color:#7c3aed;color:#fff;"`.
    Custom(&'static str),
}

impl Color {
    /// Returns the BEM modifier CSS class for this color.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Default => "code--default",
            Self::Red => "code--red",
            Self::Orange => "code--orange",
            Self::Yellow => "code--yellow",
            Self::Green => "code--green",
            Self::Teal => "code--teal",
            Self::Blue => "code--blue",
            Self::Cyan => "code--cyan",
            Self::Accent => "code--accent",
            Self::Pink => "code--pink",
            Self::Success => "code--success",
            Self::Warning => "code--warning",
            Self::Danger => "code--danger",
            Self::Custom(_) => "code--custom",
        }
    }

    /// Returns the filled (solid) inline CSS background + text color.
    pub fn to_solid_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #4b5563; color: #ffffff;",
            Self::Red => "background-color: #dc2626; color: #ffffff;",
            Self::Orange => "background-color: #ea580c; color: #ffffff;",
            Self::Yellow => "background-color: #ca8a04; color: #ffffff;",
            Self::Green => "background-color: #16a34a; color: #ffffff;",
            Self::Teal => "background-color: #0d9488; color: #ffffff;",
            Self::Blue => "background-color: #2563eb; color: #ffffff;",
            Self::Cyan => "background-color: #0891b2; color: #ffffff;",
            Self::Accent => "background-color: #7c3aed; color: #ffffff;",
            Self::Pink => "background-color: #db2777; color: #ffffff;",
            Self::Success => "background-color: #16a34a; color: #ffffff;",
            Self::Warning => "background-color: #d97706; color: #ffffff;",
            Self::Danger => "background-color: #dc2626; color: #ffffff;",
            Self::Custom(s) => s,
        }
    }

    /// Returns the outline inline CSS border + text color (no background).
    pub fn to_outline_style(self) -> &'static str {
        match self {
            Self::Default => {
                "background-color: transparent; color: #4b5563; border: 1px solid #4b5563;"
            }
            Self::Red => {
                "background-color: transparent; color: #dc2626; border: 1px solid #dc2626;"
            }
            Self::Orange => {
                "background-color: transparent; color: #ea580c; border: 1px solid #ea580c;"
            }
            Self::Yellow => {
                "background-color: transparent; color: #ca8a04; border: 1px solid #ca8a04;"
            }
            Self::Green => {
                "background-color: transparent; color: #16a34a; border: 1px solid #16a34a;"
            }
            Self::Teal => {
                "background-color: transparent; color: #0d9488; border: 1px solid #0d9488;"
            }
            Self::Blue => {
                "background-color: transparent; color: #2563eb; border: 1px solid #2563eb;"
            }
            Self::Cyan => {
                "background-color: transparent; color: #0891b2; border: 1px solid #0891b2;"
            }
            Self::Accent => {
                "background-color: transparent; color: #7c3aed; border: 1px solid #7c3aed;"
            }
            Self::Pink => {
                "background-color: transparent; color: #db2777; border: 1px solid #db2777;"
            }
            Self::Success => {
                "background-color: transparent; color: #16a34a; border: 1px solid #16a34a;"
            }
            Self::Warning => {
                "background-color: transparent; color: #d97706; border: 1px solid #d97706;"
            }
            Self::Danger => {
                "background-color: transparent; color: #dc2626; border: 1px solid #dc2626;"
            }
            Self::Custom(s) => s,
        }
    }

    /// Returns the subtle (lightly tinted) inline CSS.
    pub fn to_subtle_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #f3f4f6; color: #4b5563;",
            Self::Red => "background-color: #fee2e2; color: #dc2626;",
            Self::Orange => "background-color: #ffedd5; color: #ea580c;",
            Self::Yellow => "background-color: #fef9c3; color: #ca8a04;",
            Self::Green => "background-color: #dcfce7; color: #16a34a;",
            Self::Teal => "background-color: #ccfbf1; color: #0d9488;",
            Self::Blue => "background-color: #dbeafe; color: #2563eb;",
            Self::Cyan => "background-color: #cffafe; color: #0891b2;",
            Self::Accent => "background-color: #ede9fe; color: #7c3aed;",
            Self::Pink => "background-color: #fce7f3; color: #db2777;",
            Self::Success => "background-color: #dcfce7; color: #16a34a;",
            Self::Warning => "background-color: #fef3c7; color: #d97706;",
            Self::Danger => "background-color: #fee2e2; color: #dc2626;",
            Self::Custom(s) => s,
        }
    }

    /// Returns the surface inline CSS (slightly elevated, dark-mode friendly).
    pub fn to_surface_style(self) -> &'static str {
        match self {
            Self::Default => {
                "background-color: #e5e7eb; color: #111827; border: 1px solid #d1d5db;"
            }
            Self::Red => "background-color: #fef2f2; color: #991b1b; border: 1px solid #fca5a5;",
            Self::Orange => "background-color: #fff7ed; color: #9a3412; border: 1px solid #fdba74;",
            Self::Yellow => "background-color: #fefce8; color: #854d0e; border: 1px solid #fde047;",
            Self::Green => "background-color: #f0fdf4; color: #166534; border: 1px solid #86efac;",
            Self::Teal => "background-color: #f0fdfa; color: #115e59; border: 1px solid #5eead4;",
            Self::Blue => "background-color: #eff6ff; color: #1e40af; border: 1px solid #93c5fd;",
            Self::Cyan => "background-color: #ecfeff; color: #155e75; border: 1px solid #67e8f9;",
            Self::Accent => "background-color: #faf5ff; color: #5b21b6; border: 1px solid #c4b5fd;",
            Self::Pink => "background-color: #fdf2f8; color: #9d174d; border: 1px solid #f9a8d4;",
            Self::Success => {
                "background-color: #f0fdf4; color: #166534; border: 1px solid #86efac;"
            }
            Self::Warning => {
                "background-color: #fffbeb; color: #92400e; border: 1px solid #fcd34d;"
            }
            Self::Danger => "background-color: #fef2f2; color: #991b1b; border: 1px solid #fca5a5;",
            Self::Custom(s) => s,
        }
    }
}

/// Visual style variant of the [`Code`] component.
///
/// Controls whether inline code appears filled, outlined, subtly tinted,
/// surface-elevated, or unstyled.
///
/// # Default
///
/// [`Variant::Subtle`] is the default variant, matching Chakra UI defaults.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    Solid,
    #[default]
    Subtle,
    Outline,
    Surface,
    Plain,
    /// Fully custom, pass an arbitrary CSS class name.
    Custom(&'static str),
}

impl Variant {
    /// Returns the BEM modifier CSS class for this variant.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Solid => "code--solid",
            Self::Subtle => "code--subtle",
            Self::Outline => "code--outline",
            Self::Surface => "code--surface",
            Self::Plain => "code--plain",
            Self::Custom(s) => s,
        }
    }
}

/// Programming language hint for the [`Block`] component.
///
/// Used to display a language badge in the [`Block`] header.
/// Does not perform any syntax highlighting by itself, that is the
/// responsibility of the host application.
///
/// # Default
///
/// [`Language::Plain`] is the default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Language {
    #[default]
    Plain,
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Html,
    Css,
    Json,
    Bash,
    Toml,
    Sql,
    Go,
    Java,
    Cpp,
    C,
    Kotlin,
    Swift,
    Ruby,
    Php,
    Yaml,
    /// Fully custom language.  `label` is shown in the header badge;
    /// pass `Language::Custom("gleam")` etc.
    Custom(&'static str),
}

impl Language {
    /// Returns the display label shown in the [`Block`] header.
    pub fn to_label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Rust => "rust",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Python => "python",
            Self::Html => "html",
            Self::Css => "css",
            Self::Json => "json",
            Self::Bash => "bash",
            Self::Toml => "toml",
            Self::Sql => "sql",
            Self::Go => "go",
            Self::Java => "java",
            Self::Cpp => "c++",
            Self::C => "c",
            Self::Kotlin => "kotlin",
            Self::Swift => "swift",
            Self::Ruby => "ruby",
            Self::Php => "php",
            Self::Yaml => "yaml",
            Self::Custom(s) => s,
        }
    }

    /// Returns a BEM modifier class for this language.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Plain => "code-block--plain",
            Self::Rust => "code-block--rust",
            Self::JavaScript => "code-block--javascript",
            Self::TypeScript => "code-block--typescript",
            Self::Python => "code-block--python",
            Self::Html => "code-block--html",
            Self::Css => "code-block--css",
            Self::Json => "code-block--json",
            Self::Bash => "code-block--bash",
            Self::Toml => "code-block--toml",
            Self::Sql => "code-block--sql",
            Self::Go => "code-block--go",
            Self::Java => "code-block--java",
            Self::Cpp => "code-block--cpp",
            Self::C => "code-block--c",
            Self::Kotlin => "code-block--kotlin",
            Self::Swift => "code-block--swift",
            Self::Ruby => "code-block--ruby",
            Self::Php => "code-block--php",
            Self::Yaml => "code-block--yaml",
            Self::Custom(s) => s,
        }
    }
}

/// Returns the base inline CSS applied to every [`Code`] element.
///
/// Sets `display: inline-code`, monospace font, `border-radius`, and
/// `font-family` so the element reads immediately as code.
pub fn base_code_style() -> &'static str {
    "display: inline; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; border-radius: 4px; font-weight: 400; white-space: nowrap; vertical-align: baseline; line-height: 1.4;"
}

/// Returns the base inline CSS for the [`Block`] root container.
///
/// Sets `position: relative`, `border-radius`, and `overflow: hidden` so
/// child elements (header, content, overlay) fit flush inside it.
pub fn base_code_block_style() -> &'static str {
    "position: relative; border-radius: 8px; overflow: hidden; width: 100%; background-color: #1a1a2e; color: #e2e8f0;"
}

/// Returns the base inline CSS for the [`Header`] bar.
pub fn base_header_style() -> &'static str {
    "display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background-color: #16213e; border-bottom: 1px solid #2d3748;"
}

/// Returns the base inline CSS for the [`Content`] scrollable area.
pub fn base_content_style() -> &'static str {
    "overflow-x: auto; padding: 16px;"
}

/// Returns the base inline CSS for the [`BlockCode`] `<pre>` element.
pub fn base_pre_style() -> &'static str {
    "margin: 0; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; white-space: pre; tab-size: 2; line-height: 1.6;"
}

/// Returns the base inline CSS for the copy trigger button.
pub fn base_copy_trigger_style() -> &'static str {
    "display: inline-flex; align-items: center; justify-content: center; padding: 4px 8px; border-radius: 4px; background: transparent; border: 1px solid #4a5568; color: #a0aec0; cursor: pointer; font-size: 11px; font-family: inherit; transition: background-color 0.15s ease, color 0.15s ease; outline: none;"
}

/// Returns inline CSS for the language badge span in the header.
pub fn base_lang_badge_style() -> &'static str {
    "font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; font-size: 11px; color: #718096; text-transform: uppercase; letter-spacing: 0.05em;"
}

/// Returns the inline CSS for the title text in the header.
pub fn base_title_style() -> &'static str {
    "font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; font-size: 12px; color: #a0aec0;"
}

fn language_keywords(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::Rust => &[
            "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
            "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
            "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super",
            "trait", "true", "type", "unsafe", "use", "where", "while", "Box", "Vec", "String",
            "Option", "Result", "Some", "None", "Ok", "Err", "usize", "isize", "i8", "i16", "i32",
            "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64", "bool", "char", "str",
        ],
        Language::JavaScript | Language::TypeScript => &[
            "abstract",
            "any",
            "as",
            "async",
            "await",
            "boolean",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "debugger",
            "declare",
            "default",
            "delete",
            "do",
            "else",
            "enum",
            "export",
            "extends",
            "false",
            "finally",
            "for",
            "from",
            "function",
            "if",
            "implements",
            "import",
            "in",
            "instanceof",
            "interface",
            "let",
            "module",
            "namespace",
            "never",
            "new",
            "null",
            "number",
            "object",
            "of",
            "override",
            "package",
            "private",
            "protected",
            "public",
            "readonly",
            "return",
            "static",
            "string",
            "super",
            "switch",
            "this",
            "throw",
            "true",
            "try",
            "type",
            "typeof",
            "undefined",
            "unknown",
            "var",
            "void",
            "while",
            "with",
            "yield",
        ],
        Language::Python => &[
            "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
            "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
            "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "print",
            "raise", "return", "try", "while", "with", "yield",
        ],
        Language::Go => &[
            "break",
            "case",
            "chan",
            "const",
            "continue",
            "default",
            "defer",
            "else",
            "fallthrough",
            "for",
            "func",
            "go",
            "goto",
            "if",
            "import",
            "interface",
            "map",
            "package",
            "range",
            "return",
            "select",
            "struct",
            "switch",
            "type",
            "var",
            "nil",
            "true",
            "false",
            "make",
            "new",
            "len",
            "cap",
            "append",
            "copy",
            "delete",
            "close",
            "panic",
            "recover",
        ],
        Language::Java => &[
            "abstract",
            "assert",
            "boolean",
            "break",
            "byte",
            "case",
            "catch",
            "char",
            "class",
            "const",
            "continue",
            "default",
            "do",
            "double",
            "else",
            "enum",
            "extends",
            "final",
            "finally",
            "float",
            "for",
            "goto",
            "if",
            "implements",
            "import",
            "instanceof",
            "int",
            "interface",
            "long",
            "native",
            "new",
            "package",
            "private",
            "protected",
            "public",
            "return",
            "short",
            "static",
            "super",
            "switch",
            "synchronized",
            "this",
            "throw",
            "throws",
            "try",
            "var",
            "void",
            "volatile",
            "while",
            "true",
            "false",
            "null",
        ],
        Language::Kotlin => &[
            "abstract",
            "actual",
            "as",
            "break",
            "by",
            "catch",
            "class",
            "companion",
            "const",
            "constructor",
            "continue",
            "data",
            "do",
            "else",
            "enum",
            "expect",
            "external",
            "false",
            "final",
            "finally",
            "for",
            "fun",
            "get",
            "if",
            "import",
            "in",
            "init",
            "inline",
            "inner",
            "interface",
            "internal",
            "is",
            "it",
            "lateinit",
            "null",
            "object",
            "open",
            "operator",
            "out",
            "override",
            "package",
            "private",
            "protected",
            "public",
            "return",
            "sealed",
            "set",
            "super",
            "suspend",
            "this",
            "throw",
            "true",
            "try",
            "typealias",
            "val",
            "var",
            "vararg",
            "when",
            "where",
            "while",
        ],
        Language::Swift => &[
            "as",
            "break",
            "case",
            "catch",
            "class",
            "continue",
            "default",
            "defer",
            "deinit",
            "do",
            "else",
            "enum",
            "extension",
            "fallthrough",
            "false",
            "fileprivate",
            "final",
            "for",
            "func",
            "guard",
            "if",
            "import",
            "in",
            "init",
            "inout",
            "internal",
            "is",
            "let",
            "mutating",
            "nil",
            "open",
            "operator",
            "override",
            "private",
            "protocol",
            "public",
            "repeat",
            "required",
            "return",
            "self",
            "static",
            "struct",
            "subscript",
            "super",
            "switch",
            "throw",
            "throws",
            "true",
            "try",
            "type",
            "typealias",
            "var",
            "weak",
            "where",
            "while",
        ],
        Language::Ruby => &[
            "BEGIN", "END", "alias", "and", "begin", "break", "case", "class", "def", "defined",
            "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in", "module", "next",
            "nil", "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then",
            "true", "undef", "unless", "until", "when", "while", "yield",
        ],
        Language::Php => &[
            "abstract",
            "and",
            "array",
            "as",
            "break",
            "callable",
            "case",
            "catch",
            "class",
            "clone",
            "const",
            "continue",
            "declare",
            "default",
            "die",
            "do",
            "echo",
            "else",
            "elseif",
            "empty",
            "extends",
            "false",
            "final",
            "finally",
            "fn",
            "for",
            "foreach",
            "function",
            "global",
            "goto",
            "if",
            "implements",
            "include",
            "instanceof",
            "interface",
            "isset",
            "list",
            "match",
            "namespace",
            "new",
            "not",
            "null",
            "or",
            "print",
            "private",
            "protected",
            "public",
            "readonly",
            "require",
            "return",
            "static",
            "switch",
            "throw",
            "trait",
            "true",
            "try",
            "unset",
            "use",
            "var",
            "void",
            "while",
            "xor",
            "yield",
        ],
        Language::Sql => &[
            "ADD",
            "ALL",
            "ALTER",
            "AND",
            "AS",
            "ASC",
            "AVG",
            "BETWEEN",
            "BY",
            "CASE",
            "CHECK",
            "COLUMN",
            "CONSTRAINT",
            "COUNT",
            "CREATE",
            "CROSS",
            "DATABASE",
            "DEFAULT",
            "DELETE",
            "DESC",
            "DISTINCT",
            "DROP",
            "ELSE",
            "END",
            "EXISTS",
            "FALSE",
            "FOREIGN",
            "FROM",
            "FULL",
            "GROUP",
            "HAVING",
            "IN",
            "INDEX",
            "INNER",
            "INSERT",
            "INTO",
            "IS",
            "JOIN",
            "KEY",
            "LEFT",
            "LIKE",
            "LIMIT",
            "MAX",
            "MIN",
            "NOT",
            "NULL",
            "ON",
            "OR",
            "ORDER",
            "OUTER",
            "PRIMARY",
            "REFERENCES",
            "RIGHT",
            "SELECT",
            "SET",
            "SUM",
            "TABLE",
            "THEN",
            "TOP",
            "TRANSACTION",
            "TRUE",
            "TRUNCATE",
            "UNION",
            "UNIQUE",
            "UPDATE",
            "VALUES",
            "VIEW",
            "WHEN",
            "WHERE",
            "WITH",
        ],
        Language::Bash => &[
            "case", "do", "done", "elif", "else", "esac", "fi", "for", "function", "if", "in",
            "select", "then", "time", "until", "while", "echo", "export", "local", "return",
            "exit", "source", "readonly", "break", "continue", "shift",
        ],
        Language::Css => &[
            "important",
            "auto",
            "none",
            "inherit",
            "initial",
            "unset",
            "solid",
            "dashed",
            "dotted",
            "flex",
            "grid",
            "block",
            "inline",
            "absolute",
            "relative",
            "fixed",
            "sticky",
            "hidden",
            "visible",
            "center",
            "bold",
            "normal",
            "transparent",
            "currentColor",
        ],
        _ => &[],
    }
}

#[inline]
fn html_esc(b: u8) -> Option<&'static str> {
    match b {
        b'&' => Some("&amp;"),
        b'<' => Some("&lt;"),
        b'>' => Some("&gt;"),
        _ => None,
    }
}

#[inline]
fn push_escaped(out: &mut String, bytes: &[u8]) {
    for &b in bytes {
        if let Some(e) = html_esc(b) {
            out.push_str(e);
        } else {
            out.push(b as char);
        }
    }
}

/// Per-token colour styles for a [`Theme`].
///
/// Every field is a **full inline CSS value** (no selector), e.g.
/// `"color:#c678dd;font-weight:600"`.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ThemeColors {
    pub keyword: &'static str,
    pub string_: &'static str,
    pub comment: &'static str,
    pub number: &'static str,
    pub function_: &'static str,
    pub type_: &'static str,
    pub constant: &'static str,
    pub operator: &'static str,
    pub macro_: &'static str,
    pub decorator: &'static str,
    pub tag: &'static str,
    pub attr_name: &'static str,
    /// Main editor background colour (hex string, e.g. `"#1a1a2e"`).
    pub background: &'static str,
    /// Default foreground / text colour.
    pub foreground: &'static str,
    /// Slightly darkened background for the [`Header`] bar.
    pub header_bg: &'static str,
}

/// Built-in syntax-highlight themes for [`Block`].
///
/// Pass as the `theme` prop on [`Block`] to change the colour scheme.
#[derive(Debug, Clone, PartialEq, Copy, Default)]
#[allow(clippy::large_enum_variant)]
pub enum Theme {
    /// Atom One Dark (default)
    #[default]
    OneDark,
    Dracula,
    NightOwl,
    GithubDark,
    GithubLight,
    Solarized,
    Monokai,
    Nord,
    Custom(ThemeColors),
}

impl Theme {
    pub fn to_colors(self) -> ThemeColors {
        match self {
            Theme::OneDark => ThemeColors {
                keyword: "color:#c678dd;font-weight:600",
                string_: "color:#98c379",
                comment: "color:#5c6370;font-style:italic",
                number: "color:#d19a66",
                function_: "color:#61afef",
                type_: "color:#e5c07b",
                constant: "color:#e06c75",
                operator: "color:#56b6c2",
                macro_: "color:#e06c75;font-weight:600",
                decorator: "color:#61afef;font-style:italic",
                tag: "color:#e06c75",
                attr_name: "color:#d19a66",
                background: "#1a1a2e",
                foreground: "#e2e8f0",
                header_bg: "#16213e",
            },
            Theme::Dracula => ThemeColors {
                keyword: "color:#ff79c6;font-weight:600",
                string_: "color:#f1fa8c",
                comment: "color:#6272a4;font-style:italic",
                number: "color:#bd93f9",
                function_: "color:#50fa7b",
                type_: "color:#8be9fd",
                constant: "color:#ffb86c",
                operator: "color:#ff79c6",
                macro_: "color:#ffb86c;font-weight:600",
                decorator: "color:#50fa7b;font-style:italic",
                tag: "color:#ff79c6",
                attr_name: "color:#ffb86c",
                background: "#282a36",
                foreground: "#f8f8f2",
                header_bg: "#21222c",
            },
            Theme::NightOwl => ThemeColors {
                keyword: "color:#c792ea;font-weight:600",
                string_: "color:#ecc48d",
                comment: "color:#637777;font-style:italic",
                number: "color:#f78c6c",
                function_: "color:#82aaff",
                type_: "color:#ffcb8b",
                constant: "color:#7fdbca",
                operator: "color:#89ddff",
                macro_: "color:#addb67;font-weight:600",
                decorator: "color:#82aaff;font-style:italic",
                tag: "color:#ef5350",
                attr_name: "color:#addb67",
                background: "#011627",
                foreground: "#d6deeb",
                header_bg: "#01111f",
            },
            Theme::GithubDark => ThemeColors {
                keyword: "color:#ff7b72;font-weight:600",
                string_: "color:#a5d6ff",
                comment: "color:#8b949e;font-style:italic",
                number: "color:#79c0ff",
                function_: "color:#d2a8ff",
                type_: "color:#ffa657",
                constant: "color:#e3b341",
                operator: "color:#79c0ff",
                macro_: "color:#ff7b72;font-weight:600",
                decorator: "color:#d2a8ff;font-style:italic",
                tag: "color:#7ee787",
                attr_name: "color:#79c0ff",
                background: "#0d1117",
                foreground: "#e6edf3",
                header_bg: "#161b22",
            },
            Theme::GithubLight => ThemeColors {
                keyword: "color:#cf222e;font-weight:600",
                string_: "color:#0a3069",
                comment: "color:#6e7781;font-style:italic",
                number: "color:#0550ae",
                function_: "color:#8250df",
                type_: "color:#953800",
                constant: "color:#0550ae",
                operator: "color:#0550ae",
                macro_: "color:#cf222e;font-weight:600",
                decorator: "color:#8250df;font-style:italic",
                tag: "color:#116329",
                attr_name: "color:#0550ae",
                background: "#ffffff",
                foreground: "#24292f",
                header_bg: "#f6f8fa",
            },
            Theme::Solarized => ThemeColors {
                keyword: "color:#859900;font-weight:600",
                string_: "color:#2aa198",
                comment: "color:#586e75;font-style:italic",
                number: "color:#d33682",
                function_: "color:#268bd2",
                type_: "color:#b58900",
                constant: "color:#6c71c4",
                operator: "color:#cb4b16",
                macro_: "color:#d33682;font-weight:600",
                decorator: "color:#268bd2;font-style:italic",
                tag: "color:#dc322f",
                attr_name: "color:#b58900",
                background: "#002b36",
                foreground: "#839496",
                header_bg: "#00212b",
            },
            Theme::Monokai => ThemeColors {
                keyword: "color:#f92672;font-weight:600",
                string_: "color:#e6db74",
                comment: "color:#75715e;font-style:italic",
                number: "color:#ae81ff",
                function_: "color:#a6e22e",
                type_: "color:#66d9e8",
                constant: "color:#66d9e8",
                operator: "color:#f8f8f2",
                macro_: "color:#f92672;font-weight:600",
                decorator: "color:#a6e22e;font-style:italic",
                tag: "color:#f92672",
                attr_name: "color:#a6e22e",
                background: "#272822",
                foreground: "#f8f8f2",
                header_bg: "#1e1f1c",
            },
            Theme::Nord => ThemeColors {
                keyword: "color:#81a1c1;font-weight:600",
                string_: "color:#a3be8c",
                comment: "color:#4c566a;font-style:italic",
                number: "color:#b48ead",
                function_: "color:#88c0d0",
                type_: "color:#8fbcbb",
                constant: "color:#d08770",
                operator: "color:#81a1c1",
                macro_: "color:#bf616a;font-weight:600",
                decorator: "color:#5e81ac;font-style:italic",
                tag: "color:#bf616a",
                attr_name: "color:#ebcb8b",
                background: "#2e3440",
                foreground: "#d8dee9",
                header_bg: "#242933",
            },
            Theme::Custom(c) => c,
        }
    }

    pub fn background(self) -> &'static str {
        self.to_colors().background
    }
    pub fn foreground(self) -> &'static str {
        self.to_colors().foreground
    }
    pub fn header_bg(self) -> &'static str {
        self.to_colors().header_bg
    }
}

/// Returns the full code-block container inline CSS for the given [`Theme`].
///
/// Background and foreground are derived from the them.
pub fn themed_block_style(theme: Theme) -> String {
    let c = theme.to_colors();
    format!(
        "position:relative;border-radius:8px;overflow:hidden;width:100%;background-color:{};color:{}",
        c.background, c.foreground
    )
}

/// Returns the header bar inline CSS for the given [`Theme`].
///
/// Uses the theme's `header_bg` colour so the header is always visually
/// distinct from the code area without any external stylesheet.
pub fn themed_header_style(theme: Theme) -> String {
    let c = theme.to_colors();
    format!(
        "display:flex;align-items:center;justify-content:space-between;padding:8px 12px;background-color:{};border-bottom:1px solid rgba(128,128,128,0.15);",
        c.header_bg
    )
}

/// Returns the `<pre>` element inline CSS for the given [`Theme`].
///
/// Foreground colour is sourced from the theme so text is always readable
/// regardless of the chosen theme background.
pub fn themed_pre_style(theme: Theme) -> String {
    let c = theme.to_colors();
    format!(
        "margin:0;font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,'Liberation Mono','Courier New',monospace;white-space:pre;tab-size:2;line-height:1.6;color:{}",
        c.foreground
    )
}

fn push_span(out: &mut String, style: &str, bytes: &[u8]) {
    out.push_str("<span style=\"");
    out.push_str(style);
    out.push_str("\">");
    push_escaped(out, bytes);
    out.push_str("</span>");
}

/// Tokenises `code` for `lang` using `theme` and returns an HTML string
/// with inline-styled `<span>` elements, no external stylesheet needed.
///
/// # Examples
///
/// ```rust
/// use coders::{Language, Theme, highlight_code};
///
/// let html = highlight_code("fn main() {}", Language::Rust, Theme::OneDark);
/// assert!(html.contains("color:#c678dd"));
/// ```
pub fn highlight_code(code: &str, lang: Language, theme: Theme) -> String {
    let c = theme.to_colors();
    let kws = language_keywords(lang);
    let src = code.as_bytes();
    let n = src.len();
    let mut out = String::with_capacity(code.len() * 3);
    let mut i = 0;

    while i < n {
        let has_block = matches!(
            lang,
            Language::Rust
                | Language::JavaScript
                | Language::TypeScript
                | Language::Go
                | Language::Java
                | Language::Kotlin
                | Language::Swift
                | Language::Cpp
                | Language::C
                | Language::Php
                | Language::Css
        );
        if has_block && i + 1 < n && src[i] == b'/' && src[i + 1] == b'*' {
            let start = i;
            i += 2;
            while i + 1 < n && !(src[i] == b'*' && src[i + 1] == b'/') {
                i += 1;
            }
            if i + 1 < n {
                i += 2;
            }
            push_span(&mut out, c.comment, &src[start..i]);
            continue;
        }

        if lang == Language::Html && i + 3 < n && &src[i..i + 4] == b"<!--" {
            let start = i;
            i += 4;
            while i + 2 < n && &src[i..i + 3] != b"-->" {
                i += 1;
            }
            if i + 2 < n {
                i += 3;
            }
            push_span(&mut out, c.comment, &src[start..i]);
            continue;
        }

        let is_slc = match lang {
            Language::Rust
            | Language::JavaScript
            | Language::TypeScript
            | Language::Go
            | Language::Java
            | Language::Kotlin
            | Language::Swift
            | Language::Cpp
            | Language::C
            | Language::Php
            | Language::Css => i + 1 < n && src[i] == b'/' && src[i + 1] == b'/',
            Language::Python
            | Language::Ruby
            | Language::Bash
            | Language::Toml
            | Language::Yaml => src[i] == b'#',
            Language::Sql => i + 1 < n && src[i] == b'-' && src[i + 1] == b'-',
            _ => false,
        };
        if is_slc {
            let start = i;
            while i < n && src[i] != b'\n' {
                i += 1;
            }
            push_span(&mut out, c.comment, &src[start..i]);
            continue;
        }

        if src[i] == b'"' {
            let start = i;
            i += 1;
            while i < n {
                if src[i] == b'\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if src[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push_span(&mut out, c.string_, &src[start..i]);
            continue;
        }

        if src[i] == b'\'' && !matches!(lang, Language::Rust) {
            let start = i;
            i += 1;
            while i < n {
                if src[i] == b'\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if src[i] == b'\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push_span(&mut out, c.string_, &src[start..i]);
            continue;
        }

        if src[i] == b'`' && matches!(lang, Language::JavaScript | Language::TypeScript) {
            let start = i;
            i += 1;
            while i < n {
                if src[i] == b'\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if src[i] == b'`' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push_span(&mut out, c.string_, &src[start..i]);
            continue;
        }

        if src[i].is_ascii_digit() || (src[i] == b'.' && i + 1 < n && src[i + 1].is_ascii_digit()) {
            let start = i;
            if src[i] == b'0' && i + 1 < n && (src[i + 1] == b'x' || src[i + 1] == b'X') {
                i += 2;
                while i < n && (src[i].is_ascii_hexdigit() || src[i] == b'_') {
                    i += 1;
                }
            } else {
                while i < n
                    && (src[i].is_ascii_digit()
                        || src[i] == b'.'
                        || src[i] == b'_'
                        || src[i] == b'e'
                        || src[i] == b'E')
                {
                    i += 1;
                }
                if i < n && src[i].is_ascii_alphabetic() {
                    while i < n && src[i].is_ascii_alphanumeric() {
                        i += 1;
                    }
                }
            }
            push_span(&mut out, c.number, &src[start..i]);
            continue;
        }

        if lang == Language::Html
            && src[i] == b'<'
            && i + 1 < n
            && (src[i + 1].is_ascii_alphabetic() || src[i + 1] == b'/')
        {
            push_escaped(&mut out, &src[i..i + 1]);
            i += 1;
            if i < n && src[i] == b'/' {
                out.push('/');
                i += 1;
            }
            let start = i;
            while i < n && (src[i].is_ascii_alphanumeric() || src[i] == b'-') {
                i += 1;
            }
            push_span(&mut out, c.tag, &src[start..i]);
            continue;
        }

        if src[i] == b'@'
            && matches!(
                lang,
                Language::Python | Language::Css | Language::Bash | Language::Yaml
            )
        {
            let start = i;
            i += 1;
            while i < n && (src[i].is_ascii_alphanumeric() || src[i] == b'_' || src[i] == b'-') {
                i += 1;
            }
            push_span(&mut out, c.decorator, &src[start..i]);
            continue;
        }

        if lang == Language::Bash && src[i] == b'$' {
            let start = i;
            i += 1;
            while i < n
                && (src[i].is_ascii_alphanumeric()
                    || src[i] == b'_'
                    || src[i] == b'{'
                    || src[i] == b'}')
            {
                i += 1;
            }
            push_span(&mut out, c.attr_name, &src[start..i]);
            continue;
        }

        if src[i].is_ascii_alphabetic() || src[i] == b'_' {
            let start = i;
            while i < n && (src[i].is_ascii_alphanumeric() || src[i] == b'_') {
                i += 1;
            }
            let word = &code[start..i];

            if matches!(lang, Language::Rust | Language::Cpp | Language::C)
                && i < n
                && src[i] == b'!'
                && (i + 1 >= n || src[i + 1] != b'=')
            {
                push_span(&mut out, c.macro_, &src[start..i]);
                continue;
            }

            if kws.contains(&word) {
                push_span(&mut out, c.keyword, &src[start..i]);
                continue;
            }

            let mut j = i;
            while j < n && src[j] == b' ' {
                j += 1;
            }
            if j < n && src[j] == b'(' {
                push_span(&mut out, c.function_, &src[start..i]);
                continue;
            }

            let wb = word.as_bytes();

            if wb[0].is_ascii_uppercase()
                && wb.len() > 1
                && wb[1..].iter().any(|&b| b.is_ascii_lowercase())
            {
                push_span(&mut out, c.type_, &src[start..i]);
                continue;
            }

            if wb.len() > 1
                && wb
                    .iter()
                    .all(|&b| b.is_ascii_uppercase() || b == b'_' || b.is_ascii_digit())
            {
                push_span(&mut out, c.constant, &src[start..i]);
                continue;
            }

            push_escaped(&mut out, &src[start..i]);
            continue;
        }

        if let Some(e) = html_esc(src[i]) {
            out.push_str(e);
        } else {
            out.push(src[i] as char);
        }
        i += 1;
    }

    out
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
