"""Tests for the models module."""

import pytest

import oxphys_numerics as ox


def test_init_constant():
    """Make sure that we can initialize constants."""
    # Make sure that we can initialize a constant with a value of 1.0
    constant = ox.Constant(1.0)
    assert constant.value == 1.0

    # Make sure that we get a TypeError when we try to initialize a constant with a string value.
    with pytest.raises(TypeError):
        ox.Constant("1.0")  # type: ignore


def test_init_variable():
    """Make sure that we can initialize variables."""
    # Make sure that we can initialize a variable with name "x".
    variable = ox.Variable("x")
    assert variable._name == "x"

    # Make sure that we get a TypeError when we try to initialize a variable with a float value.
    with pytest.raises(TypeError):
        ox.Variable(1.0)  # type: ignore


def test_init_negate():
    """Make sure that we can initialize negations."""
    # Make sure that we can initialize a negation implicitly.
    constant = ox.Constant(1.0)
    assert isinstance(-constant, ox.Negate)

    # Make sure that we can initialize a negation explicitly.
    ox.Negate(constant)


def test_init_sqrt():
    """Make sure that we can initialize square roots."""
    # Make sure that we can initialize a square root.
    constant = ox.Constant(1.0)
    assert isinstance(ox.Sqrt(constant), ox.Sqrt)


def test_init_sin():
    """Make sure that we can initialize sines."""
    # Make sure that we can initialize a sine.
    constant = ox.Constant(1.0)
    assert isinstance(ox.Sin(constant), ox.Sin)


def test_init_cos():
    """Make sure that we can initialize cosines."""
    # Make sure that we can initialize a cosine.
    constant = ox.Constant(1.0)
    assert isinstance(ox.Cos(constant), ox.Cos)


def test_init_exp():
    """Make sure that we can initialize exponentials."""
    # Make sure that we can initialize an exponential.
    constant = ox.Constant(1.0)
    assert isinstance(ox.Exp(constant), ox.Exp)


def test_init_ln():
    """Make sure that we can initialize natural logarithms."""
    # Make sure that we can initialize a natural logarithm.
    constant = ox.Constant(1.0)
    assert isinstance(ox.Ln(constant), ox.Ln)


def test_init_add():
    """Make sure that we can initialize additions."""
    # Make sure that we can initialize an addition implicitly.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(constant1 + constant2, ox.Add)

    # Make sure that we can initialize an addition explicitly.
    ox.Add(constant1, constant2)


def test_init_minus():
    """Make sure that we can initialize subtractions."""
    # Make sure that we can initialize a subtraction implicitly.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(constant1 - constant2, ox.Minus)

    # Make sure that we can initialize a subtraction explicitly.
    ox.Minus(constant1, constant2)


def test_init_mul():
    """Make sure that we can initialize multiplications."""
    # Make sure that we can initialize a multiplication implicitly.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(constant1 * constant2, ox.Multiply)

    # Make sure that we can initialize a multiplication explicitly.
    ox.Multiply(constant1, constant2)


def test_init_power():
    """Make sure that we can initialize powers."""
    # Make sure that we can initialize a power.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(constant1**constant2, ox.Power)

    # Make sure that we can initialize a power explicitly.
    ox.Power(constant1, constant2)


def test_init_log():
    """Make sure that we can initialize logarithms."""
    # Make sure that we can initialize a logarithm.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(ox.Log(constant1, constant2), ox.Log)


def test_init_fraction():
    """Make sure that we can initialize fractions."""
    # Make sure that we can initialize a fraction implicitly.
    constant1 = ox.Constant(1.0)
    constant2 = ox.Constant(2.0)
    assert isinstance(constant1 / constant2, ox.Fraction)

    # Make sure that we can initialize a fraction explicitly.
    ox.Fraction(constant1, constant2)
