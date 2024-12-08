"""Define all the pydantic models used to construct expressions.

We explicitly override a large number of magic methods in these classes to allow for a more
mathematically intuitive syntax when constructing expressions. For example, we override the `+`
operator to allow for the syntax `expr1 + expr2` to be used to create an `Add` expression.

Similarly, going against typical recommendations when working with pydantic models, we allow for
positional arguments to be passed to the `__init__` method of these classes.
"""

from typing import Self, Union

from pydantic import BaseModel as PydanticBaseModel

from .errors import InvalidExpressionError

ExprCastable = Union["Expr", float, int, str]
"""A type alias for the types that can be used to construct an expression."""


class BaseModel(PydanticBaseModel):
    """The base class for all oxphys_numerics pydantic models.

    This base class is here in case we want to change some global pydantic config for all models in
    the whole `oxphys_numerics` package.
    """


def _to_expr(expr: ExprCastable) -> "Expr":
    """Convert a value to an expression.

    This function can accept an expression, or anything that we can automatically convert to one.
    For example, passing a float/int will create a `Constant` expression, and passing a string will
    will create a `Variable` expression.
    """
    if isinstance(expr, Expr):
        return expr
    if isinstance(expr, float | int):
        return Constant(value=expr)
    if isinstance(expr, str):
        return Variable(name=expr)

    # If execution reaches this point, someone broke the type hints on this function and passed in
    # an unsupported type.
    raise InvalidExpressionError.from_unsupported_type(expr)


class Expr(BaseModel):
    """Represents a mathematical expression.

    This base class handles the common operator overloads etc. that are shared by all expression
    types.
    """

    def __add__(self, other: ExprCastable) -> "Add":
        """Add two expressions together.

        This method is used to allow the syntax `expr1 + expr2` to be used to
        create an `Add` expression.
        """
        return Add(left=self, right=_to_expr(other))

    def __radd__(self, other: ExprCastable) -> "Add":
        """Add two expressions together.

        This method is used to allow the syntax `expr1 + expr2` to be used to
        create an `Add` expression.
        """
        return Add(left=_to_expr(other), right=self)

    def __sub__(self, other: ExprCastable) -> "Sub":
        """Subtract one expression from another.

        This method is used to allow the syntax `expr1 - expr2` to be used to
        create a `Sub` expression.
        """
        return Sub(left=self, right=_to_expr(other))

    def __rsub__(self, other: ExprCastable) -> "Sub":
        """Subtract one expression from another.

        This method is used to allow the syntax `expr1 - expr2` to be used to
        create a `Sub` expression.
        """
        return Sub(left=_to_expr(other), right=self)

    def __mul__(self, other: ExprCastable) -> "Mul":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Mul(left=self, right=_to_expr(other))

    def __rmul__(self, other: ExprCastable) -> "Mul":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Mul(left=_to_expr(other), right=self)

    def __truediv__(self, other: ExprCastable) -> "Div":
        """Divide one expression by another.

        This method is used to allow the syntax `expr1 / expr2` to be used to
        create a `Div` expression.
        """
        return Div(left=self, right=_to_expr(other))

    def __rtruediv__(self, other: ExprCastable) -> "Div":
        """Divide one expression by another.

        This method is used to allow the syntax `expr1 / expr2` to be used to
        create a `Div` expression.
        """
        return Div(left=_to_expr(other), right=self)

    def __neg__(self) -> "Negate":
        """Negate an expression.

        This method is used to allow the syntax `-expr` to be used to create a
        `Negate` expression.
        """
        return Negate(expr=self)

    def __pos__(self) -> Self:
        """Return the expression unchanged.

        This method is used to allow the syntax `+expr` to be used to return the
        expression unchanged.
        """
        return self

    def __pow__(self, other: ExprCastable) -> "Exp":
        """Raise an expression to a power.

        This method is used to allow the syntax `expr1 ** expr2` to be used to
        create an `Exp` expression.
        """
        return Exp(expr=self, power=_to_expr(other))

    def __rpow__(self, other: ExprCastable) -> "Exp":
        """Raise an expression to a power.

        This method is used to allow the syntax `expr1 ** expr2` to be used to
        create an `Exp` expression.
        """
        return Exp(expr=_to_expr(other), power=self)


# region Leaf nodes


class Leaf(Expr):
    """The base class for all leaf nodes in the expression tree."""


class Constant(Leaf):
    """Represents a constant value."""

    value: float


class Variable(Leaf):
    """Represents a variable."""

    name: str


# endregion
# region Unary nodes


class Unary(Expr):
    """The base class for all unary nodes in the expression tree."""

    expr: Expr

    def __init__(self, expr: ExprCastable, /) -> None:
        """Initialise a new unary expression."""
        super().__init__(expr=_to_expr(expr))


class Negate(Expr):
    """Represents the negation of an expression."""


class Sqrt(Expr):
    """Represents the square root of an expression."""


class Sin(Expr):
    """Represents the sine of an expression."""


class Cos(Expr):
    """Represents the cosine of an expression."""


class Exp(Expr):
    """Represents the exponential of an expression."""


class Ln(Expr):
    """Represents the natural logarithm of an expression."""


# endregion
# region Binary nodes


class Binary(Expr):
    """The base class for all binary nodes in the expression tree."""

    left: Expr
    right: Expr

    def __init__(self, left: ExprCastable, right: ExprCastable, /) -> None:
        """Initialise a new binary expression.

        Args:
            left: The first input to the binary expression.
            right: The second input to the binary expression.
        """
        super().__init__(left=_to_expr(left), right=_to_expr(right))


class Add(Expr):
    """Represents the addition of two expressions."""


class Sub(Expr):
    """Represents the subtraction of two expressions."""


class Mul(Expr):
    """Represents the multiplication of two expressions."""


class Log(Expr):
    """Represents the logarithm of an expression to a given base."""

    def __init__(self, arg: ExprCastable, base: ExprCastable, /) -> None:
        """Initialise a new logarithm expression.

        Args:
            arg: The argument of the logarithm.
            base: The base of the logarithm.
        """
        super().__init__(arg=_to_expr(arg), base=_to_expr(base))


class Div(Expr):
    """Represents the division of two expressions."""


# endregion
