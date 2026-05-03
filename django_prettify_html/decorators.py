import functools


def no_html_prettification(view):
    """Decorator to skip HTML prettification for a specific view.

    Apply to views that produce HTML which should not be prettified:

        from django_prettify_html.decorators import no_html_prettification

        @no_html_prettification
        def raw_html_view(request):
            return HttpResponse("<pre>raw output</pre>")
    """

    @functools.wraps(view)
    def wrapper(*args, **kwargs):
        response = view(*args, **kwargs)
        response._no_prettification = True
        return response

    return wrapper
