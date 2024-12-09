"""Define all the data models used to construct expressions.

We explicitly override a large number of magic methods in these classes to allow for a more
mathematically intuitive syntax when constructing expressions. For example, we override the `+`
operator to allow for the syntax `expr1 + expr2` to be used to create an `Add` expression.
"""

import abc
from typing import TYPE_CHECKING, Self, Union, overload

import numpy as np
import pandas as pd
import polars as pl

from .errors import InvalidExpressionError

if TYPE_CHECKING:
    from collections.abc import Sequence

ExprCastable = Union["Expr", float, int, str]
"""A type alias for the types that can be used to construct an expression."""

NumLike = float | int
"""A type alias for the types that can be used to represent a number."""

ArrayLike = Union["np.ndarray", "Sequence[NumLike]"]
"""A type alias for the types that can be used to represent an array-like object."""

VariableId = Union[str, "Variable"]
"""A type alias for the types that can be used to identify a variable."""

ExprCallArg = Union[
    "dict[VariableId, NumLike]",
    "dict[VariableId, ArrayLike]",
    "pd.DataFrame",
    "pl.DataFrame",
    None,
]
"""A type alias for the types that can be passed to the `__call__` method of an expression."""


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


# region Expr


class Expr(abc.ABC):
    """Represents a mathematical expression.

    This base class handles the common operator overloads etc. that are shared by all expression
    types.
    """

    def __add__(self, other: ExprCastable) -> "Add":
        """Add two expressions together.

        This method is used to allow the syntax `expr1 + expr2` to be used to
        create an `Add` expression.
        """
        return Add(self, _to_expr(other))

    def __radd__(self, other: ExprCastable) -> "Add":
        """Add two expressions together.

        This method is used to allow the syntax `expr1 + expr2` to be used to
        create an `Add` expression.
        """
        return Add(_to_expr(other), self)

    def __sub__(self, other: ExprCastable) -> "Minus":
        """Subtract one expression from another.

        This method is used to allow the syntax `expr1 - expr2` to be used to
        create a `Sub` expression.
        """
        return Minus(self, _to_expr(other))

    def __rsub__(self, other: ExprCastable) -> "Minus":
        """Subtract one expression from another.

        This method is used to allow the syntax `expr1 - expr2` to be used to
        create a `Sub` expression.
        """
        return Minus(_to_expr(other), self)

    def __mul__(self, other: ExprCastable) -> "Mul":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Mul(self, _to_expr(other))

    def __rmul__(self, other: ExprCastable) -> "Mul":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Mul(_to_expr(other), self)

    def __truediv__(self, other: ExprCastable) -> "Fraction":
        """Divide one expression by another.

        This method is used to allow the syntax `expr1 / expr2` to be used to
        create a `Div` expression.
        """
        return Fraction(numerator=self, denominator=_to_expr(other))

    def __rtruediv__(self, other: ExprCastable) -> "Fraction":
        """Divide one expression by another.

        This method is used to allow the syntax `expr1 / expr2` to be used to
        create a `Div` expression.
        """
        return Fraction(numerator=_to_expr(other), denominator=self)

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

    def __pow__(self, other: ExprCastable) -> "Power":
        """Raise an expression to a power.

        This method is used to allow the syntax `expr1 ** expr2` to be used to
        create an `Exp` expression.
        """
        return Power(base=self, exponent=_to_expr(other))

    def __rpow__(self, other: ExprCastable) -> "Power":
        """Raise an expression to a power.

        This method is used to allow the syntax `expr1 ** expr2` to be used to
        create an `Exp` expression.
        """
        return Power(base=_to_expr(other), exponent=self)

    def __str__(self) -> str:
        """Create the string representation of the expression."""
        return self.to_latex()

    @overload
    def __call__(self, data: None = None, **kwargs: NumLike) -> float: ...

    @overload
    def __call__(self, data: None = None, **kwargs: ArrayLike) -> np.ndarray: ...

    @overload
    def __call__(self, data: "pd.DataFrame") -> "pd.DataFrame": ...

    @overload
    def __call__(self, data: "pl.DataFrame") -> "pl.DataFrame": ...

    @overload
    def __call__(self, data: "dict[VariableId, NumLike]") -> float: ...

    @overload
    def __call__(self, data: "dict[VariableId, ArrayLike]") -> np.ndarray: ...

    def __call__(
        self,
        data: ExprCallArg = None,
        **kwargs: NumLike | ArrayLike,
    ) -> "float | np.ndarray | pd.DataFrame | pl.DataFrame":
        """Evaluate the expression."""
        # If we were given a pandas DataFrame, convert it to polars.
        if isinstance(data, pd.DataFrame):
            data = pl.from_pandas(data)

        # If we were given a polars dataframe, convert it to a dictionary.
        if isinstance(data, pl.DataFrame):
            data = {col: data[col].to_numpy() for col in data.columns}

        # Our input data is the data if provided, or the kwargs if not.
        data_dict = data or kwargs

        # Get the first value from the dictionary.
        first_value = next(iter(data_dict.values()))

        # Check to see if we've got one value for each variable.
        if isinstance(first_value, int | float):
            # If the first value is an int | float, then all the values must be int | float.
            if not all(isinstance(value, int | float) for value in data_dict.values()):
                raise InvalidExpressionError.from_inconsistent_arguments(data_dict)

            # We need a type ignore here because this type check is tricky for it to follow.
            return self.call_float(data_dict)  # type: ignore

        # If execution reaches here, we must have all array-like values. We need the type ignore
        # because type checkers struggle with the above isinstance checks.
        return self.call_array(data_dict)  # type: ignore

    def call_float(self, data: "dict[Variable, NumLike]") -> float:
        """Evaluate the expression when each variable has a single value.

        This method must be implemented by all expressions to allow for the evaluation of the
        expression with a dictionary of variables.
        """
        raise NotImplementedError

    def call_array(self, data: "dict[Variable, ArrayLike]") -> np.ndarray | float:
        """Evaluate the expression many times; each variable has an array of values.

        This method must be implemented by all expressions to allow for the evaluation of the
        expression with a dictionary of arrays.
        """
        raise NotImplementedError

    @abc.abstractmethod
    def to_latex(self) -> str:
        """Convert the expression to a LaTeX string.

        This method must be implemented by all expressions to allow for the conversion of the
        expression to a LaTeX string.

        If nothing else, it's handy for debugging/visualization, and it's easy enough to do.
        """

    @abc.abstractmethod
    def variables(self) -> list["Variable"]:
        """Return a list of all the variables in the expression.

        This method must be implemented by all expressions to allow for the extraction of all the
        variables in the expression.
        """


# endregion
# region Leaf


class Leaf(Expr):
    """The base class for all leaf nodes in the expression tree."""


class Constant(Leaf):
    """Represents a constant value."""

    def __init__(self, value: float) -> None:
        """Initialise a new constant.

        Args:
            value: The value of the constant.
        """
        self._value = value

    @property
    def value(self) -> float:
        """Return the value of the constant."""
        return self._value

    def to_latex(self) -> str:
        # Make sure that we recast the value to an integer if it's actually an int under the hood.
        # This happens quite a lot when someone makes an expression using something like:
        # `Constant(2) + Constant(3)`.
        value = int(self.value) if self.value.is_integer() else self.value
        return str(value)

    def variables(self) -> list["Variable"]:
        return []


class Variable(Leaf):
    """Represents a variable."""

    def __init__(self, name: str) -> None:
        """Initialise a new variable.

        Args:
            name: The name of the variable.
        """
        self._name = name

    def to_latex(self) -> str:
        return self._name

    def variables(self) -> list["Variable"]:
        return [self]

    def __str__(self) -> str:
        """Create the string representation of the variable.

        If we're just printing a variable, instead of defaulting to the LaTeX representation, we
        have a slightly nicer, more custom representation.
        """
        return f"Variable({self._name})"

    def __eq__(self, other: object) -> bool:
        """Check if two variables are equal.

        This is necessary for the variable to be used in sets and as a key in dictionaries.
        """
        # If it isn't a variable, it can't be equal.
        if not isinstance(other, Variable):
            return False

        # If the names are the same, the variables are equal.
        return self._name == other._name

    def __hash__(self) -> int:
        """Return the hash of the variable.

        This is necessary for the variable to be used in sets and as a key in dictionaries.
        """
        return hash(self._name)


# endregion
# region Unary


class Unary(Expr):
    """The base class for all unary nodes in the expression tree."""

    def __init__(self, expr: ExprCastable) -> None:
        """Initialise a new unary expression."""
        self._expr = _to_expr(expr)

    def variables(self) -> list["Variable"]:
        """Return a list of all the variables in the expression."""
        return self._expr.variables()


class Negate(Unary):
    """Represents the negation of an expression."""

    def to_latex(self) -> str:
        return f"-{self._expr.to_latex()}"


class Sqrt(Unary):
    """Represents the square root of an expression."""

    def to_latex(self) -> str:
        return R"\sqrt{" + self._expr.to_latex() + "}"


class Sin(Unary):
    """Represents the sine of an expression."""

    def to_latex(self) -> str:
        return R"\sin{ \left(" + self._expr.to_latex() + R"\right) }"


class Cos(Unary):
    """Represents the cosine of an expression."""

    def to_latex(self) -> str:
        return R"\cos{ \left(" + self._expr.to_latex() + R"\right) }"


class Exp(Unary):
    """Represents the exponential of an expression."""

    def to_latex(self) -> str:
        return R"e^{" + self._expr.to_latex() + "}"


class Ln(Unary):
    """Represents the natural logarithm of an expression."""

    def to_latex(self) -> str:
        return R"\ln{" + self._expr.to_latex() + "}"


# endregion
# region Binary


class Binary(Expr):
    """The base class for all binary nodes in the expression tree."""

    def __init__(self, left: ExprCastable, right: ExprCastable) -> None:
        """Initialise a new binary expression.

        Args:
            left: The first input to the binary expression.
            right: The second input to the binary expression.
        """
        self._left = _to_expr(left)
        self._right = _to_expr(right)

    def variables(self) -> list["Variable"]:
        """Return a list of all the variables in the expression."""
        # We need to make sure that we don't have any duplicates in the list of variables.
        return list(set(self._left.variables() + self._right.variables()))


class Add(Binary):
    """Represents the addition of two expressions."""

    def to_latex(self) -> str:
        return f"{self._left.to_latex()} + {self._right.to_latex()}"


class Minus(Binary):
    """Represents the subtraction of two expressions."""

    def to_latex(self) -> str:
        return f"{self._left.to_latex()} - {self._right.to_latex()}"


class Mul(Binary):
    """Represents the multiplication of two expressions."""

    def to_latex(self) -> str:
        # Note that we don't put a \times here; I think it leads to an uglier output.
        return f"{self._left.to_latex()} {self._right.to_latex()}"


class Power(Binary):
    """Represents the power of two expressions."""

    def __init__(self, base: ExprCastable, exponent: ExprCastable) -> None:
        """Initialise a new power expression.

        Args:
            base: The base of the power.
            exponent: The exponent of the power.
        """
        # This sets self.left to base and self.right to exponent.
        super().__init__(base, exponent)

    @property
    def base(self) -> Expr:
        return self._left

    @property
    def exponent(self) -> Expr:
        return self._right

    def to_latex(self) -> str:
        return f"{self.base.to_latex()}^{{{self.exponent.to_latex()}}}"


class Log(Binary):
    """Represents the logarithm of an expression to a given base."""

    def __init__(self, arg: ExprCastable, base: ExprCastable) -> None:
        """Initialise a new logarithm expression.

        Args:
            arg: The argument of the logarithm.
            base: The base of the logarithm.
        """
        # This sets self.left to arg and self.right to base.
        super().__init__(arg, base)

    @property
    def arg(self) -> Expr:
        """Return the argument of the logarithm."""
        return self._left

    @property
    def base(self) -> Expr:
        """Return the base of the logarithm."""
        return self._right

    def to_latex(self) -> str:
        return R"\log_{" + self.base.to_latex() + R"}{ \left(" + self.arg.to_latex() + R"\right)}"


class Fraction(Binary):
    """Represents the division of two expressions."""

    def __init__(self, numerator: ExprCastable, denominator: ExprCastable) -> None:
        """Initialise a new division expression.

        Args:
            numerator: The numerator of the division.
            denominator: The denominator of the division.
        """
        # This sets self.left to numerator and self.right to denominator.
        super().__init__(numerator, denominator)

    @property
    def numerator(self) -> Expr:
        """Return the numerator of the division."""
        return self._left

    @property
    def denominator(self) -> Expr:
        """Return the denominator of the division."""
        return self._right

    def to_latex(self) -> str:
        return R"\frac{" + self._left.to_latex() + "}{" + self._right.to_latex() + "}"


# endregion
