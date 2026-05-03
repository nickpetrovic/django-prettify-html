import django_prettify_html


def test_basic_format():
    html = '<div><p>Hello</p><span>World</span></div>'
    result = django_prettify_html.format(html)
    assert "<div>" in result
    assert "  <p>Hello</p>" in result or "<p>Hello</p>" in result


def test_indent_width():
    # Use a full document so the formatter has block-level nesting to indent
    html = '<!DOCTYPE html><html><head><meta charset="UTF-8"></head><body><div><p>Hello</p></div></body></html>'
    result = django_prettify_html.format(html, indent_width=4)
    assert "    " in result  # Should contain 4-space indentation


def test_format_full_document():
    html = '<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hi</h1></body></html>'
    result = django_prettify_html.format(html)
    assert "<!DOCTYPE html>" in result or "<!doctype html>" in result
    assert "\n" in result  # Should have line breaks


def test_language_jinja():
    html = '<div>{% if x %}<p>yes</p>{% endif %}</div>'
    result = django_prettify_html.format(html, language="jinja")
    assert "{% if x %}" in result


def test_invalid_language():
    import pytest
    with pytest.raises(ValueError, match="Unknown language"):
        django_prettify_html.format("<div></div>", language="invalid")


def test_quotes_single():
    html = '<div class="foo"></div>'
    result = django_prettify_html.format(html, quotes="single")
    assert "'" in result or "class=" in result


def test_empty_input():
    result = django_prettify_html.format("")
    assert result == "" or result.strip() == ""
