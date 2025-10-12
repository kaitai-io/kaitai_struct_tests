const std = @import("std");
const testing = std.testing;
const format_imports_with_syntax_error = @import("../formats/format_imports_with_syntax_error.zig");

test "FormatImportsWithSyntaxError" {
    _ = format_imports_with_syntax_error.FormatImportsWithSyntaxError.create();
}
