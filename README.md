# Kaitai Struct: specs and tests

This repository contains specifications and tests for 
[Kaitai Structures](https://github.com/kaitai-io/kaitai_struct) project.

## What's inside

The repository is laid out like that:

* `src/` - binary input files that would be parsed during the tests
* `formats/` - file formats description is Kaitai Struct YAML format
  for the files in `src/`
* `spec/` - specifications (i.e. test code) that uses format
  descriptions to parse binary input files and ensures that they're
  parsed properly.

During the testing the following is expected to be created:

* `compiled/` - formats (described in `formats/`), compiled into specific programming languages modules
  * `java/` - in Java
  * `python/` - in Python
  * `ruby/` - in Ruby

## How to test

The overall procedure of testing works as follows:

* Compile format descriptions in `formats/` into source files in
  relevant programming languages (Java, Python, Ruby, etc), which
  should be placed in `compiled/$LANGUAGE`.
* Compile and run test code for particular language (located in
  `spec/$LANGUAGE`), which will use files in `src/` for input.

## Automated test tools

There are a few scripts that automate steps specified above:

* `run-$LANGUAGE` executes all tests for a particular `$LANGUAGE` using
  preferred language-specific testing tool.

Obviously, these scripts require Kaitai Struct compiler and
language-specific runtime modules. They are normally located in
distinct repositories - there is a file named `/config` that specifies
their default locations. If you've used [main Kaitai Structures
project](https://github.com/kaitai-io/kaitai_struct) to check out all
linked repositories as submodules in their default locations, you
don't need to adjust anything. If you have these dependencies in some
other places, just edit the `/config`.
