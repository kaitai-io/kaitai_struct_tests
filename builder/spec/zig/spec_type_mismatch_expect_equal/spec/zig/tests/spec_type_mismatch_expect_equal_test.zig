const std = @import("std");
const testing = std.testing;

test "SpecTypeMismatchExpectEqual" {
    testing.expectEqual("3", 3);
}
