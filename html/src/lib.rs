use proc_macro_hack::proc_macro_hack;

pub use yew_html_common::html_tree::HtmlTree;

/// Generate html tree
#[proc_macro_hack]
pub use yew_html_impl::html;
