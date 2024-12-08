"""Define the OxphysNumericsError exception."""


class OxphysNumericsError(ValueError):
    """The base exception for all oxphys_numerics errors.

    Users can catch this exception to catch all errors that are emitted by the oxphys_numerics
    library.
    """
