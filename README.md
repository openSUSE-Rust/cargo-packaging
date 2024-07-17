
## Cargo Packaging Macros and Tools for OpenSUSE

This is a set of macros and tools to help make rust/cargo packaging a bit nicer.

### Macros

These are used in your .spec file to build, test and install your rust application.

     %{cargo_build}
     %{cargo_test}
     %{cargo_install}

### Custom RUSTFLAGS

Custom RUSTFLAGS can be set with:

    %define __rustflags --cfg tokio_unstable

### Tools

These tools are used both manually and automatically in your build to help generate
metadata for your package.

#### Manual

#### Automatic






