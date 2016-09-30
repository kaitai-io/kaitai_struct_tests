# Kaitai Struct: specs and tests

This repository contains specifications and tests for 
[Kaitai Struct](https://github.com/kaitai-io/kaitai_struct) project.

## What's inside

The repository is laid out like that:

* `src/` - binary input files that would be parsed during the tests
* `formats/` - file formats description is Kaitai Struct YAML format
  for the files in `src/`
* `spec/` - specifications (i.e. test code) that uses format
  descriptions to parse binary input files and ensures that they're
  parsed properly.
  * `$LANGUAGE/` - one subdirectory per every supported target language

During the testing the following is expected to be created:

* `compiled/` - formats (described in `formats/`), compiled into specific programming languages modules
  * `$LANGUAGE/` - one subdirectory per every supported target language
* `test_out/` - test running output, in a language-specific format
  * `$LANGUAGE/` - one subdirectory per every supported target language

## How to test

The overall procedure of testing works as follows:

* Make sure that KS compiler (ksc) is built and ready to be used
* Compile format descriptions in `formats/` into source files in
  relevant programming languages (Java, Python, Ruby, etc), which
  should be placed in `compiled/$LANGUAGE`.
* Compile and run test code for particular language (located in
  `spec/$LANGUAGE`), which will use files in `src/` for input.
* Aggregate and view results

## Automated test tools

There are a few scripts that automate steps specified above:

* `build-compiler` builds compiler using special "stage" mode,
  i.e. without system-wide deployment, ready to be run from a build
  directory
* `build-formats` compiles all format descriptions in `formats/` with
  this compiler for every supported language, placing results in
  `compiled/$LANGUAGE`
* `run-$LANGUAGE` executes all tests for a particular `$LANGUAGE` using
  preferred language-specific testing tool.
* `run-all`
  * runs tests for all languages (usually by executing
    `run-$LANGUAGE`, sometimes with special arguments for brevity and
    proper test machine-readable output generation)
  * aggregates all testing results into single nice HTML page by
    running `aggregate/aggregate` result aggregation & formatting
    script

Obviously, these scripts require Kaitai Struct compiler and
language-specific runtime modules. They are normally located in
distinct repositories - there is a file named `/config` that specifies
their default locations. If you've used [main Kaitai Struct project](https://github.com/kaitai-io/kaitai_struct)
to check out all linked repositories as submodules in their default
locations, you don't need to adjust anything. If you have these
dependencies in some other places, just edit the `/config`.

## Continuous integration

[Main Kaitai Struct project](https://github.com/kaitai-io/kaitai_struct)
includes a [Travis CI](https://travis-ci.org/kaitai-io/kaitai_struct/)
configuration. This mean that every commit to main project repository
gets automatically built and tested throughly. The results are
published at [Kaitai Struct CI results page](http://kaitai.io/ci).
