// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/coders/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/opensass/coders/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;

pub mod common;

pub use common::{
    Color, Language, Size, Theme, ThemeColors, Variant, highlight_code, themed_block_style,
    themed_header_style, themed_pre_style,
};

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
