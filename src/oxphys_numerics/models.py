"""Define all the data models used to construct expressions.

We explicitly override a large number of magic methods in these classes to allow for a more
mathematically intuitive syntax when constructing expressions. For example, we override the `+`
operator to allow for the syntax `expr1 + expr2` to be used to create an `Add` expression.
"""

import abc
from collections.abc import Sequence
from typing import Self, Union, overload

import numpy as np
import oxphys_numerics_rs as rs
import pandas as pd
import polars as pl

from .errors import InvalidExpressionError

_RustExpr = rs.PyExpr  # type: ignore
"""The rust expression type that we're wrapping up here."""


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

    def __init__(self) -> None:
        """Initialise a new expression."""
        self._rs_expr = None

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

    def __mul__(self, other: ExprCastable) -> "Multiply":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Multiply(self, _to_expr(other))

    def __rmul__(self, other: ExprCastable) -> "Multiply":
        """Multiply two expressions together.

        This method is used to allow the syntax `expr1 * expr2` to be used to
        create a `Mul` expression.
        """
        return Multiply(_to_expr(other), self)

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
    def __call__(self, data: None = None, *, parallel: bool = True, **kwargs: NumLike) -> float: ...

    @overload
    def __call__(
        self, data: None = None, *, parallel: bool = True, **kwargs: ArrayLike
    ) -> np.ndarray: ...

    @overload
    def __call__(
        self,
        data: "pd.DataFrame",
        *,
        parallel: bool = True,
    ) -> "pd.DataFrame": ...

    @overload
    def __call__(
        self,
        data: "pl.DataFrame",
        *,
        parallel: bool = True,
    ) -> "pl.DataFrame": ...

    @overload
    def __call__(
        self,
        data: "dict[VariableId, NumLike]",
        *,
        parallel: bool = True,
    ) -> float: ...

    @overload
    def __call__(
        self,
        data: "dict[VariableId, ArrayLike]",
        *,
        parallel: bool = True,
    ) -> np.ndarray: ...

    def __call__(
        self,
        data: ExprCallArg = None,
        *,
        parallel: bool = True,
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
        return self.call_array(data_dict, parallel=parallel)  # type: ignore

    def call_float(self, data: "dict[str | Variable, NumLike]") -> float:
        """Evaluate the expression when each variable has a single value.

        This method must be implemented by all expressions to allow for the evaluation of the
        expression with a dictionary of variables.
        """
        raise NotImplementedError

    def call_array(self, data: "dict[str | Variable, ArrayLike]", *, parallel: bool) -> np.ndarray:
        """Evaluate the expression many times; each variable has an array of values.

        This method must be implemented by all expressions to allow for the evaluation of the
        expression with a dictionary of arrays.
        """
        # Make sure that the underlying rust expression is built.
        if self._rs_expr is None:
            parameter_list = self._build_rust_parameter_list([])
            self._rs_expr = self._build_rust_expr(parameter_list)

        # Get the parameter list, so we know what order to expect the variables in.
        parameter_list = self._build_rust_parameter_list([])

        # Remake the data dict, but keyed by the variable name.
        data_str_keys = {
            key.name if isinstance(key, Variable) else key: value for key, value in data.items()
        }

        # Now we need to build a 2D array of the data, with one column for each variable, where
        # column 0 is the first variable in the parameter list, column 1 is the second, etc.
        data_array = np.array([data_str_keys[var] for var in parameter_list]).T

        # Call the evaluate_vec rust method on the rust expression.
        return self._rs_expr.evaluate_vec(data_array, parallel)  # type: ignore

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

    @abc.abstractmethod
    def children(self) -> list["Expr"]:
        """Return a list of all the children of the expression."""

    @abc.abstractmethod
    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        """Build the rust expression from the python expression.

        This method must be implemented by all expressions to allow for the construction of the
        rust expression from the python expression. This isn't type annotated because it's we can't
        know the type of the rust expression using python static type checkers.

        This must return an instance of _RustExpr.

        Args:
            parameter_list: The variable names that we've found so far while building the
                expression. This is used to decide where each parameter should go where in the
                compiled expression.
        """

    @abc.abstractmethod
    def _build_rust_parameter_list(self, parameter_list_builder: list[str]) -> list[str]:
        """Build the rust parameter list from the python expression.

        This method must be implemented by all expressions to allow for the construction of the
        rust parameter list from the python expression.
        This is needed by the jit compiler to know the order it should expect the parameters in, and
        on the python end to know what order to pass them in.

        This method needs to be deterministic, so that we can call it multiple times and always be
        able to get the correct parameter list.
        """


# endregion
# region Leaf


class Leaf(Expr):
    """The base class for all leaf nodes in the expression tree."""

    def children(self) -> list[Expr]:
        # Leaf nodes have no children.
        return []


class Constant(Leaf):
    """Represents a constant value."""

    def __init__(self, value: float) -> None:
        """Initialise a new constant.

        Args:
            value: The value of the constant.
        """
        self._value = value
        super().__init__()

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

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        del parameter_list  # We don't need this for constants.
        return _RustExpr.constant(self.value)

    def _build_rust_parameter_list(self, parameter_list_builder: list[str]) -> list[str]:
        return parameter_list_builder


class Variable(Leaf):
    """Represents a variable."""

    def __init__(self, name: str) -> None:
        """Initialise a new variable.

        Args:
            name: The name of the variable.
        """
        self._name = name
        super().__init__()

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

    @property
    def name(self) -> str:
        """The name of this variable."""
        return self._name

    def to_latex(self) -> str:
        return self._name

    def variables(self) -> list["Variable"]:
        return [self]

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Find the index of the variable in the parameter list.
        index = parameter_list.index(self._name)

        # We need to let the rust expression know what index in the parameter list the variable is.
        return _RustExpr.variable(index)

    def _build_rust_parameter_list(self, parameter_list_builder: list[str]) -> list[str]:
        # If the variable isn't already in the list, add it.
        if self._name not in parameter_list_builder:
            parameter_list_builder.append(self._name)

        # Now return the potentially updated list.
        return parameter_list_builder


# endregion
# region Unary


class Unary(Expr):
    """The base class for all unary nodes in the expression tree."""

    def __init__(self, expr: ExprCastable) -> None:
        """Initialise a new unary expression."""
        self._expr = _to_expr(expr)
        super().__init__()

    def variables(self) -> list["Variable"]:
        """Return a list of all the variables in the expression."""
        return self._expr.variables()

    def children(self) -> list[Expr]:
        return [self._expr]

    def _build_rust_parameter_list(self, parameter_list_builder: list[str]) -> list[str]:
        # Recursively build the parameter list for the inner expression.
        return self._expr._build_rust_parameter_list(parameter_list_builder)


class Negate(Unary):
    """Represents the negation of an expression."""

    def to_latex(self) -> str:
        return f"-{self._expr.to_latex()}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Negate the inner expression.
        return _RustExpr.negate(_inner_rs_expr)


class Sqrt(Unary):
    """Represents the square root of an expression."""

    def to_latex(self) -> str:
        return R"\sqrt{" + self._expr.to_latex() + "}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Take the square root of the inner expression.
        return _RustExpr.sqrt(_inner_rs_expr)


class Sin(Unary):
    """Represents the sine of an expression."""

    def to_latex(self) -> str:
        return R"\sin{ \left(" + self._expr.to_latex() + R"\right) }"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Take the sine of the inner expression.
        return _RustExpr.sin(_inner_rs_expr)


class Cos(Unary):
    """Represents the cosine of an expression."""

    def to_latex(self) -> str:
        return R"\cos{ \left(" + self._expr.to_latex() + R"\right) }"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Take the cosine of the inner expression.
        return _RustExpr.cos(_inner_rs_expr)


class Exp(Unary):
    """Represents the exponential of an expression."""

    def to_latex(self) -> str:
        return R"e^{" + self._expr.to_latex() + "}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Take the exponential of the inner expression.
        return _RustExpr.exp(_inner_rs_expr)


class Ln(Unary):
    """Represents the natural logarithm of an expression."""

    def to_latex(self) -> str:
        return R"\ln{" + self._expr.to_latex() + "}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the inner expression.
        _inner_rs_expr = self._expr._build_rust_expr(parameter_list)

        # Take the natural logarithm of the inner expression.
        return _RustExpr.ln(_inner_rs_expr)


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
        super().__init__()

    def variables(self) -> list["Variable"]:
        """Return a list of all the variables in the expression."""
        # We need to make sure that we don't have any duplicates in the list of variables.
        return list(set(self._left.variables() + self._right.variables()))

    def children(self) -> list[Expr]:
        return [self._left, self._right]

    def _build_rust_parameter_list(self, parameter_list_builder: list[str]) -> list[str]:
        # Recursively build the parameter list for the left and right expressions.
        parameter_list_builder = self._left._build_rust_parameter_list(parameter_list_builder)
        return self._right._build_rust_parameter_list(parameter_list_builder)


class Add(Binary):
    """Represents the addition of two expressions."""

    def to_latex(self) -> str:
        return f"{self._left.to_latex()} + {self._right.to_latex()}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self._left._build_rust_expr(parameter_list)
        _right_rs_expr = self._right._build_rust_expr(parameter_list)

        # Add the left and right expressions.
        return _RustExpr.add(_left_rs_expr, _right_rs_expr)


class Minus(Binary):
    """Represents the subtraction of two expressions."""

    def to_latex(self) -> str:
        return f"{self._left.to_latex()} - {self._right.to_latex()}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self._left._build_rust_expr(parameter_list)
        _right_rs_expr = self._right._build_rust_expr(parameter_list)

        # Subtract the right expression from the left expression.
        return _RustExpr.subtract(_left_rs_expr, _right_rs_expr)


class Multiply(Binary):
    """Represents the multiplication of two expressions."""

    def to_latex(self) -> str:
        # Note that we don't put a \times here; I think it leads to an uglier output.
        return f"{self._left.to_latex()} {self._right.to_latex()}"

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self._left._build_rust_expr(parameter_list)
        _right_rs_expr = self._right._build_rust_expr(parameter_list)

        # Multiply the left and right expressions.
        return _RustExpr.multiply(_left_rs_expr, _right_rs_expr)


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

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self.base._build_rust_expr(parameter_list)
        _right_rs_expr = self.exponent._build_rust_expr(parameter_list)

        # Raise the left expression to the power of the right expression.
        return _RustExpr.pow(_left_rs_expr, _right_rs_expr)


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

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self.arg._build_rust_expr(parameter_list)
        _right_rs_expr = self.base._build_rust_expr(parameter_list)

        # Take the logarithm of the left expression with the right expression as the base.
        return _RustExpr.log(_left_rs_expr, _right_rs_expr)


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

    def _build_rust_expr(self, parameter_list: Sequence[str]):  # noqa: ANN202
        # Make the left and right expressions.
        _left_rs_expr = self.numerator._build_rust_expr(parameter_list)
        _right_rs_expr = self.denominator._build_rust_expr(parameter_list)

        # Divide the left expression by the right expression.
        return _RustExpr.frac(_left_rs_expr, _right_rs_expr)


# endregion
