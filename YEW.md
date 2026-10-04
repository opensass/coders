# Y Yew Code RS

## Installation

```sh
cargo add coders --features yew
```

## Usage

```rust
use coders::yew::{Code, Block, Header, Title, CopyTrigger, Content, LanguageBadge};
use coders::{Color, Language, Size, Variant};
use yew::prelude::*;
```

## Inline Code

```rust
use coders::yew::Code;
use coders::{Color, Size, Variant};
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Code color={Color::Accent} variant={Variant::Subtle} size={Size::Sm}>
            {"console.log(\"Hello, world!\")"}
        </Code>
    }
}
```

## Code Block (Basic)

```rust
use coders::yew::{Block, Content, Header, Title, CopyTrigger};
use coders::Language;
use yew::prelude::*;

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <Block code="fn main() {\n    println!(\"Hello, world!\");\n}" language={Language::Rust}>
            <Header>
                <Title>{"main.rs"}</Title>
                <CopyTrigger />
            </Header>
            <Content />
        </Block>
    }
}
```

## Props

### `Code`

| Prop          | Default           | Type           | Description                            |
| ------------- | ----------------- | -------------- | -------------------------------------- |
| `color`       | `Color::Default`  | `Color`        | Color palette applied to the component |
| `variant`     | `Variant::Subtle` | `Variant`      | Visual style variant                   |
| `size`        | `Size::Sm`        | `Size`         | Font size and padding                  |
| `class`       | `""`              | `&'static str` | Additional CSS class names             |
| `style`       | `""`              | `&'static str` | Inline CSS override                    |
| `id`          | `""`              | `&'static str` | HTML `id` attribute                    |
| `aria_label`  | `""`              | `&'static str` | Accessible label                       |
| `data_testid` | `""`              | `&'static str` | Test identifier                        |

### `Block`

| Prop          | Default           | Type           | Description                 |
| ------------- | ----------------- | -------------- | --------------------------- |
| `code`        | -                 | `&'static str` | Raw code content (required) |
| `language`    | `Language::Plain` | `Language`     | Language hint for badge     |
| `class`       | `""`              | `&'static str` | Additional CSS class names  |
| `style`       | `""`              | `&'static str` | Inline CSS override         |
| `id`          | `""`              | `&'static str` | HTML `id` attribute         |
| `aria_label`  | `"Code block"`    | `&'static str` | Accessible region label     |
| `data_testid` | `""`              | `&'static str` | Test identifier             |

### `CopyTrigger`

| Prop           | Default                    | Type           | Description                 |
| -------------- | -------------------------- | -------------- | --------------------------- |
| `copy_label`   | `"Copy"`                   | `&'static str` | Label before copying        |
| `copied_label` | `"Copied!"`                | `&'static str` | Label after copy            |
| `reset_ms`     | `2000`                     | `u32`          | Reset delay in milliseconds |
| `aria_label`   | `"Copy code to clipboard"` | `&'static str` | Accessible label            |

### `Color` variants

`Default`, `Red`, `Orange`, `Yellow`, `Green`, `Teal`, `Blue`, `Cyan`, `Accent`, `Pink`, `Success`, `Warning`, `Danger`

### `Variant` variants

`Solid`, `Subtle`, `Outline`, `Surface`, `Plain`

### `Size` variants

`Xs`, `Sm`, `Md`, `Lg`, `Custom(&'static str)`

### `Language` variants

`Plain`, `Rust`, `JavaScript`, `TypeScript`, `Python`, `Html`, `Css`, `Json`, `Bash`, `Toml`, `Sql`, `Go`, `Java`, `Cpp`, `C`, `Kotlin`, `Swift`, `Ruby`, `Php`, `Yaml`, `Custom(&'static str)`

### `Theme` variants

`OneDark` _(default)_, `Dracula`, `NightOwl`, `GithubDark`, `GithubLight`, `Solarized`, `Monokai`, `Nord`, `Custom(ThemeColors)`

## See Also

- [MDN: `<pre>`, Preformatted Text element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/pre): The semantic root of every `Content` rendering.
- [MDN: `<code>`, Inline Code element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code): Used for both the inline `Code` component and the highlighted code node inside `Content`.
- [MDN: `<figure>`, Figure with Optional Caption element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure): The landmark wrapper rendered by `Block`.
- [MDN: `Clipboard.writeText()`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText): The Web API called by `CopyTrigger` to copy code to the clipboard.
