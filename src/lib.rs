use markup_fmt::config::{
    DoctypeKeywordCase, FormatOptions, LanguageOptions, LayoutOptions, LineBreak, Quotes,
    WhitespaceSensitivity,
};
use markup_fmt::{format_text, Language};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::borrow::Cow;
use std::num::NonZeroUsize;

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(
    signature = (
        code,
        *,
        language = "html",
        print_width = 80,
        use_tabs = false,
        indent_width = 2,
        line_break = "lf",
        quotes = "double",
        format_comments = false,
        closing_bracket_same_line = false,
        max_attrs_per_line = None,
        prefer_attrs_single_line = false,
        html_normal_self_closing = None,
        html_void_self_closing = None,
        whitespace_sensitivity = "css",
        doctype_keyword_case = "ignore"
    )
)]
fn format(
    code: &str,
    language: &str,
    print_width: usize,
    use_tabs: bool,
    indent_width: usize,
    line_break: &str,
    quotes: &str,
    format_comments: bool,
    closing_bracket_same_line: bool,
    max_attrs_per_line: Option<usize>,
    prefer_attrs_single_line: bool,
    html_normal_self_closing: Option<bool>,
    html_void_self_closing: Option<bool>,
    whitespace_sensitivity: &str,
    doctype_keyword_case: &str,
) -> PyResult<String> {
    let lang = match language {
        "html" => Language::Html,
        "vue" => Language::Vue,
        "svelte" => Language::Svelte,
        "astro" => Language::Astro,
        "angular" => Language::Angular,
        "jinja" => Language::Jinja,
        "vento" => Language::Vento,
        "mustache" => Language::Mustache,
        "xml" => Language::Xml,
        _ => return Err(PyValueError::new_err(format!("Unknown language: {language}"))),
    };

    let lb = match line_break {
        "lf" => LineBreak::Lf,
        "crlf" => LineBreak::Crlf,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown line_break: {line_break}"
            )))
        }
    };

    let q = match quotes {
        "double" => Quotes::Double,
        "single" => Quotes::Single,
        _ => return Err(PyValueError::new_err(format!("Unknown quotes: {quotes}"))),
    };

    let ws = match whitespace_sensitivity {
        "css" => WhitespaceSensitivity::Css,
        "strict" => WhitespaceSensitivity::Strict,
        "ignore" => WhitespaceSensitivity::Ignore,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown whitespace_sensitivity: {whitespace_sensitivity}"
            )))
        }
    };

    let dk = match doctype_keyword_case {
        "ignore" => DoctypeKeywordCase::Ignore,
        "upper" => DoctypeKeywordCase::Upper,
        "lower" => DoctypeKeywordCase::Lower,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown doctype_keyword_case: {doctype_keyword_case}"
            )))
        }
    };

    let options = FormatOptions {
        layout: LayoutOptions {
            print_width,
            use_tabs,
            indent_width,
            line_break: lb,
        },
        language: LanguageOptions {
            quotes: q,
            format_comments,
            closing_bracket_same_line,
            max_attrs_per_line: max_attrs_per_line.and_then(NonZeroUsize::new),
            prefer_attrs_single_line,
            html_normal_self_closing,
            html_void_self_closing,
            whitespace_sensitivity: ws,
            doctype_keyword_case: dk,
            ..Default::default()
        },
    };

    // No external formatter for script/style tags — pass through unchanged
    format_text(code, lang, &options, |src, _hints| {
        Ok::<_, std::convert::Infallible>(Cow::Borrowed(src))
    })
    .map_err(|e| PyValueError::new_err(format!("Format error: {e}")))
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format, m)?)?;
    Ok(())
}
