"""
django-prettify-html: Use markup_fmt, the extremely fast HTML formatter, with Django.

Provides Python bindings for the Rust markup_fmt library via PyO3,
plus a Django middleware for automatic HTML prettification.
"""

from importlib.metadata import version

from django_prettify_html._native import format

__all__ = ["format"]
__version__ = version("django-prettify-html")
