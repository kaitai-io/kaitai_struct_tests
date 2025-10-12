const std = @import("std");
const testing = std.testing;
const format_imports_with_type_mismatch = @import("../formats/format_imports_with_type_mismatch.zig");

test "FormatImportsWithTypeMismatch" {
    _ = format_imports_with_type_mismatch.FormatImportsWithTypeMismatch.create();
}
