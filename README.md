# oxphys_numerics

To build the python package as a module in your current virtual environment, first go to the `core/pybindings` directory, then run

```bash
uv run --project ../.. maturin develop --uv
```

The reason for the slightly weird command is that `maturin` is only a dependency in the root project, but maturin only knows how to build the package if it's run in the same directory as the package.
As a result, we first go to the directory of the project that we want to build, but then point `uv` to the root project.
