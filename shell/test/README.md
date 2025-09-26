# Shell Wrapper Testing

There's no easy way to automate testing of the shell wrappers so this is the next best thing.
In this directory you'll find helper scripts for each supported shell.
Additionally, there's a set of mappings that should cover all of the insert types as well as the eval and execute features of `leadr`.

The source_me scripts set the `leadr` config directory to this directory and source the appropriate shell wrapper.
Unlike the builtin version, this does not parse the config-defined leadr binding but simply replaces the template with `\C-g`.
This allows iterating on the shell wrappers without having to recompile the rust binary (assuming the rust side works as expected).

Source the appropriate script for your shell and then run each of the defined mappings.
Make sure the observed behavior matches the expected behavior.
If you need to adjust the wrapper make sure to re-source the source_me script before running the mappings again.
