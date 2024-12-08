"""All exceptions defined by the `oxphys_numerics` package are defined in this package."""

from ._invalid_expression_error import InvalidExpressionError
from ._oxphys_numerics_error import OxphysNumericsError

__all__ = [
    "InvalidExpressionError",
    "OxphysNumericsError",
]
