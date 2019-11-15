This is a unified framework for building projects for compiled
languages, supporting partially good compilation.

## The problem

Kaitai Struct tests sometimes are not good up to 100% (actually, the
sad reality is that most of the time they are below 100%). It's not
only the problem that happens in runtime: quite often, it's even
impossible to parse / compile a certain generated file.

Some languages rely on the project being completely parseable and
compilable even before trying to build a certain final test binary and
being able to run it. Typical candidates: C++, C#, Java, Go, Rust.

## The solution

This framework offers a unified approach to achieve "partial"
compilation for such languages. In a nutshell, what it does is a
simple loop:

* Identify files to be included in a compilation
* Try to build it
* If not successful:
  * Identify offending files (by parsing build log)
  * Remove them from a list of files to compile
  * Retry build

The loop stops either when:

* We've found a certain subset of files which compiles fine =>
  that's a full or partial success
* It's impossible to remove more files and/or removing more files
  does not improve the situation => that's a complete failure.

## Implementation details

Framework is implemented in Ruby. Main class is PartialBuilder in `partial_builder.rb`.

Adding support for new language typically involves creation of
`lang_builder.rb`, which inherits PartialBuilder and implements a few
abstract methods there.
