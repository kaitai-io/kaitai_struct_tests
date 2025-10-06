const std = @import("std");
const testing = std.testing;
const spec_type_mismatch_format_param = @import("../formats/spec_type_mismatch_format_param.zig");

test "SpecTypeMismatchFormatParam" {
    _ = spec_type_mismatch_format_param.SpecTypeMismatchFormatParam.create(5);
}
