# django-prettify-html

Use [markup_fmt](https://github.com/g-plane/markup_fmt), the extremely fast HTML formatter, with Django.

Provides a Django middleware that automatically prettifies HTML responses for
readable "View Source" output during development — the counterpart to
[django-minify-html](https://github.com/adamchainz/django-minify-html) for production.

Powered by Rust via [PyO3](https://github.com/PyO3/pyo3) — no subprocess overhead, no Node.js dependency.

## Requirements

- Python 3.10 to 3.14
- Django 4.2 to 6.0

## Installation

```bash
pip install django-prettify-html
```

## Setup

1. Add to your `INSTALLED_APPS`:

```python
INSTALLED_APPS = [
    ...,
    "django_prettify_html",
    ...,
]
```

2. Add the middleware (typically last, or in place of a minification middleware):

```python
MIDDLEWARE = [
    ...,
    "django_prettify_html.middleware.PrettifyHtmlMiddleware",
]
```

## Usage

The middleware prettifies all non-streaming, non-encoded HTML responses.

### Customizing options

Subclass the middleware and override `format_args`:

```python
from django_prettify_html.middleware import PrettifyHtmlMiddleware

class ProjectPrettifyHtmlMiddleware(PrettifyHtmlMiddleware):
    format_args = PrettifyHtmlMiddleware.format_args | {
        "indent_width": 4,
        "print_width": 100,
    }
```

### Skipping specific views

```python
from django_prettify_html.decorators import no_html_prettification

@no_html_prettification
def raw_view(request):
    return HttpResponse("<pre>unformatted</pre>")
```

### Using the formatter directly

```python
from django_prettify_html import format

html = "<div><p>Hello</p></div>"
pretty = format(html, indent_width=2, print_width=80)
```

### Available options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `language` | str | `"html"` | Language: html, jinja, vue, svelte, astro, angular, vento, mustache, xml |
| `print_width` | int | `80` | Maximum line width before wrapping |
| `indent_width` | int | `2` | Number of spaces per indent level |
| `use_tabs` | bool | `False` | Use tabs instead of spaces |
| `line_break` | str | `"lf"` | Line break style: "lf" or "crlf" |
| `quotes` | str | `"double"` | Attribute quote style: "double" or "single" |
| `format_comments` | bool | `False` | Format HTML comments |
| `closing_bracket_same_line` | bool | `False` | Keep closing bracket on same line as last attribute |
| `max_attrs_per_line` | int\|None | `None` | Maximum attributes per line (None = unlimited) |
| `prefer_attrs_single_line` | bool | `False` | Prefer all attributes on a single line when possible |
| `html_normal_self_closing` | bool\|None | `None` | Self-close normal HTML elements |
| `html_void_self_closing` | bool\|None | `None` | Self-close void elements (br, img, etc.) |
| `whitespace_sensitivity` | str | `"css"` | Whitespace handling: "css", "strict", or "ignore" |
| `doctype_keyword_case` | str | `"ignore"` | DOCTYPE keyword case: "ignore", "upper", or "lower" |

## Recommended setup: minify in production, prettify in development

```python
# settings/base.py
MIDDLEWARE = [
    ...,
    "myapp.middleware.MinifyMiddleware",  # production minification
]

# settings/development.py
MIDDLEWARE = [
    "django_prettify_html.middleware.PrettifyHtmlMiddleware" if m == "myapp.middleware.MinifyMiddleware" else m
    for m in MIDDLEWARE
]
```

## Releasing

Version is defined in one place: `Cargo.toml`. Both `pyproject.toml` and `__init__.__version__` read from it automatically.

To release a new version:

```bash
# 1. Bump version in Cargo.toml
# 2. Commit, tag, and push
git commit -am "v0.2.0"
git tag v0.2.0
git push origin main --tags
```

The GitHub Actions release workflow handles the rest:
- Builds wheels for Linux (x86/arm64), macOS (x86/arm64), Windows (x86)
- Builds for Python 3.10, 3.11, 3.12, 3.13, 3.14
- Publishes to PyPI via trusted publishing
- Creates a GitHub Release with auto-generated notes and wheel artifacts

## License

MIT
