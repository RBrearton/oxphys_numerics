# oxphys_numerics

***`oxphys_numerics` is currently under active development.***

To build the python package as a module in your current virtual environment, first go to the `core/pybindings` directory, then run

```bash
uv run --project ../.. maturin develop --uv
```

The reason for the slightly weird command is that `maturin` is only a dependency in the root project, but maturin only knows how to build the package if it's run in the same directory as the package.
As a result, we first go to the directory of the project that we want to build, but then point `uv` to the root project.

Run the tests with

```bash
uv run pytest
```

## Jit compilation

The `oxphys_numerics` package's python api lets users define functions using an expression graph.
That expression graph, when evaluated, is *just-in-time* compiled on the rust side to produce a function that's as optimal as if you had hand-written it in rust.

The following are some notes for developers on jit compilation.

### Writing a jit compiler

We use the [cranelift](https://cranelift.dev/) library to help us jit compile the expression graph.
I'd like to find time to write a more substantial article on compilers, but for now, the main procedure is as follows:

#### Define the function signature

This step should be relatively easy to understand, as far as these things go.
This involves specifying a calling convention, the parameters, the return type, and the function name.

#### Build the function intermediate representation (IR)

Now we're more in the weeds.
Compilers are normally divided into four parts:

##### 1. Frontend

This is the part of the compiler that takes the source code and turns it into an intermediate representation.
That's simple enough - now we just need to define what's meant by "intermediate representation"!

##### 2. intermediate representation

This is a language independent representation of the code.
By building intermediate representations, compiler developers can write language independent optimizations, which is super powerful.
If you aren't regularly thinking about compilers, the intermediate representation will probably be quite tricky to understand.
Also, because each compiler will have their own IR, these things tend to be slightly less well documented than you might hope.

The cranelift intermediate representation does have documentation, but as of the writing of this file, it's quite brief and not hosted on a website I'd like to provide a link to (it's currently a markdown file in the cranelift repo...)
Figuring out how this works is a bit of a pain if you're new to compiler development, but hopefully the code in this repo will be enough to get you started.

##### 3. Optimizer

Should be self-explanatory.
Given the IR, the optimizer will apply a series of transformations to the IR to make it run faster!
We shouldn't need to worry about this step - cranelift will handle it for us.

##### 4. Backend

This is the part of the compiler that takes the optimized IR and turns it into machine code that can be run on the target platform (like x86 or ARM).
Again, cranelift will handle this for us.

#### Back to the IR

Now that we know a bit more about how a compiler is structure, you should be able to appreciate that most of what we'll need to do to write a jit compiler is build the IR.
As mentioned above, this isn't so well documented, but hopefully the code in this repository will be enough to get you off the ground.

Defining the IR is the first thing that we need to do after defining the function signature, and it's by far the most important/complex job that you'll have to do when writing a jit compiler.

#### Finalize

Once you've set up the IR, you need to finalize the function.
This involves actually compiling the IR into machine code.
This is quite straight forward.

#### Transmute

The final step is to transmute the function pointer that's produced by the finalization procedure into a piece of code that we can call from rust.
Again, this is quite routine - most of our work was in setting up the IR.
