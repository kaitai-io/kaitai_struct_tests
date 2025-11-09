# This test was created to reproduce a bug in the generated Zig code, where
# local constants used for imports clashed with instances of the same names. If
# the generated code used the `const std = @import("std");` statement in the
# global scope (which is common in Zig), references to `std` from within the
# struct definition would fail to compile with `error: ambiguous reference` due
# to the presence of the instance getter `pub fn std(self: *...)`. The same
# applies to other imports, such as `const integers = @import("integers.zig");`.
meta:
  id: name_clash_import_vs_inst
  imports:
    - integers
instances:
  integers:
    pos: 0
    type: integers
  std:
    value: 1 + 2
