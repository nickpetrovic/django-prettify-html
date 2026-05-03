from django.http import HttpRequest, HttpResponse

from django_prettify_html import format


class PrettifyHtmlMiddleware:
    """Django middleware that prettifies HTML responses using markup_fmt.

    Add to your MIDDLEWARE setting. The middleware runs format() on the
    content of HTML responses, producing clean, indented output suitable
    for development "View Source" debugging.

    To customize formatting options, subclass and override format_args:

        class ProjectPrettifyHtmlMiddleware(PrettifyHtmlMiddleware):
            format_args = PrettifyHtmlMiddleware.format_args | {
                "indent_width": 4,
            }

    To skip prettification on specific views, use @no_html_prettification.
    To control per-request, override should_prettify().
    """

    format_args: dict = {
        "indent_width": 2,
        "print_width": 120,
    }

    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request: HttpRequest) -> HttpResponse:
        response = self.get_response(request)
        if self.should_prettify(request, response):
            try:
                decoded = response.content.decode(response.charset or "utf-8")
                formatted = format(decoded, **self.format_args)
                response.content = formatted.encode(response.charset or "utf-8")
            except (ValueError, UnicodeDecodeError):
                pass  # Fall through with original content on format errors
        return response

    def should_prettify(self, request: HttpRequest, response: HttpResponse) -> bool:
        if getattr(response, "_no_prettification", False):
            return False
        content_type = response.get("Content-Type", "")
        if "text/html" not in content_type:
            return False
        if getattr(response, "streaming", False):
            return False
        if response.get("Content-Encoding"):
            return False
        return True
