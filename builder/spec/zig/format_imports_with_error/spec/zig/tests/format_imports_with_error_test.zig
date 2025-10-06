const std = @import("std");
const testing = std.testing;
const format_imports_with_error = @import("../formats/format_imports_with_error.zig");

test "FormatImportsWithError" {
    _ = format_imports_with_error.FormatImportsWithError.create();
}
