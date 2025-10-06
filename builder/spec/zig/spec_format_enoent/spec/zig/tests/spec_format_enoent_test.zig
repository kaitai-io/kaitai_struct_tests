const std = @import("std");
const testing = std.testing;
const spec_format_enoent = @import("../formats/spec_format_enoent.zig");

test "SpecFormatEnoent" {
    _ = spec_format_enoent.SpecFormatEnoent;
}
