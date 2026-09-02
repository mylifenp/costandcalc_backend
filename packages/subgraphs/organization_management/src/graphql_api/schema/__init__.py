from typing import TYPE_CHECKING

if TYPE_CHECKING:
    # Expose these names to type checkers without forcing runtime imports
    from .domain import Domain  # noqa: F401
    from .group import Group  # noqa: F401
    from .member import Member  # noqa: F401
    from .organization import Organization  # noqa: F401
    from .user import User  # noqa: F401


__all__ = ["Domain", "Group", "Member", "Organization", "User"]


def __getattr__(name: str):
    """Lazy import schema types to avoid circular imports at module import time.

    Accessing `graphql_api.schema.Member` will import `graphql_api.schema.member`
    on demand and return the `Member` symbol.
    """
    if name in __all__:
        module = __import__(f"{__name__}.{name.lower()}", fromlist=[name])
        return getattr(module, name)
    raise AttributeError(name)


def __dir__():
    return __all__
