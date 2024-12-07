"""Define the InvalidExpressionError exception."""

from typing import Any, Self

from ._oxphys_numerics_error import OxphysNumericsError


class InvalidExpressionError(OxphysNumericsError):
    """Raised when an invalid expression is encountered."""

    @classmethod
    def from_unsupported_type(cls, obj: Any) -> Self:  # noqa: ANN401
        """Raise this error when we can't make an expression because we don't support the type."""
        return cls(f"Cannot convert {obj} with type {type(obj)} to an expression.")
