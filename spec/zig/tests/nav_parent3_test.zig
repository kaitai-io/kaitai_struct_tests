const std = @import("std");
const kaitai_struct = @import("kaitai_struct");
const nav_parent3 = @import("../formats/nav_parent3.zig");

test "NavParent3" {
    const file = try std.fs.cwd().openFile("../../src/nav_parent2.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try nav_parent3.NavParent3.create(&arena, &_io, null, null);
    try std.testing.expectEqual(8, r.ofs_tags);
    try std.testing.expectEqual(2, r.num_tags);
    try std.testing.expectEqualStrings("RAHC", (try r.tags()).items[0].name);
    try std.testing.expectEqual(32, (try r.tags()).items[0].ofs);
    try std.testing.expectEqual(3, (try r.tags()).items[0].num_items);
    try std.testing.expectEqualStrings("foo", (try (try r.tags()).items[0].tagContent()).?.content);
    try std.testing.expectEqualStrings("RAHC", (try r.tags()).items[1].name);
    try std.testing.expectEqual(35, (try r.tags()).items[1].ofs);
    try std.testing.expectEqual(6, (try r.tags()).items[1].num_items);
    try std.testing.expectEqualStrings("barbaz", (try (try r.tags()).items[1].tagContent()).?.content);
}
