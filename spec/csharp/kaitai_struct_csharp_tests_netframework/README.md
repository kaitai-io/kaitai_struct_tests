This directory hosts "old style" project for running Kaitai Struct C#
tests using .NET Framework 4.8.

Notable points:

* We strive hard not to use `dotnet` family of commands in this mode
  to maximum compatibility with older .NET build systems.
* Dependencies are managed with `packages.config` outside of csproj file.
* Dependencies has to be downloaded (restored) with `nuget restore`
  (modern `dotnet restore` does not support dependencies in deprecated
  `packages.config` format)
* Tests are ran by executing NUnit console runner .exe manually (and thus
  either require Windows to run .exe directly, or require Mono to run
  .exe with `mono` runner)
