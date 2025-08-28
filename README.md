# Kaitai Struct: specs and tests

This repository contains specifications and tests for
[Kaitai Struct](https://github.com/kaitai-io/kaitai_struct) project.

## What's inside

The repository is laid out like that:

* `aggregate` - scripts that converts result of tests into unified `ci.json` output,
  which is used by https://ci.kaitai.io/ as source of data.
* `builder` - the same as above
* `formats/` - file formats description is Kaitai Struct YAML format
  for the files in `src/`
* `formats_err/` - file formats description is Kaitai Struct YAML format which
  is expected to generate compiler error. The `ksc` output is located in the comment
  at the beginning of the file
* `spec/` - specifications (i.e. test code) that uses format
  descriptions to parse binary input files and ensures that they're
  parsed properly.
  * `ks/` - subdirectory containing language agnostic tests in KST format
  * `$LANGUAGE/` - one subdirectory per every supported target language.
    Most of tests are generated from `spec/ks/` files using Kaitai Struct Test Translator
* `src/` - binary input files that would be parsed during the tests
* `translator/` - source code of the Kaitai Struct Test Translator utility which
  is used to generate language specific tests from language agnostic KSY descriptions.

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
  directory. It also publish it to the local repository, where in can be
  found be dependent projects, especially by the Kaitai Struct Test Translator
  (located in the `translator/` subdirectory)
* `build-tests` generates test files for the selected languages from language
  agnostic test description in KST format into corresponding test files in target
  language. It depends on the latest features of the compiler, so it is necessary
  to build compiler first using `build-compiler` script. The important part here
  that the compiler should be published to the local repository which the mentioned
  script do
* `build-formats` compiles all format descriptions in `formats/` with
  this compiler for every supported language, placing results in
  `compiled/$LANGUAGE`
* `run-$LANGUAGE` executes all tests for a particular `$LANGUAGE`
  using preferred language-specific testing tool. The output is
  generally dumped on screen for quick assessment during development.
* `ci-$LANGUAGE` also runs all tests for a particular `$LANGUAGE`, but
  logs all output into designated log file instead (mostly useful for
  aggregation within a CI system afterwards).

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
published at [Kaitai Struct CI results page](https://ci.kaitai.io).

Please refer to [CI documentation](https://doc.kaitai.io/ci.html) for
a throughout overview of how this all is tied together in a bigger
picture.
