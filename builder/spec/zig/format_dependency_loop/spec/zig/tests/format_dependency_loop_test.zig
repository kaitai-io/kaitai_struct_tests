const std = @import("std");
const testing = std.testing;
const format_dependency_loop = @import("../formats/format_dependency_loop.zig");

test "FormatDependencyLoop" {
    _ = try format_dependency_loop.FormatDependencyLoop.create();
}
