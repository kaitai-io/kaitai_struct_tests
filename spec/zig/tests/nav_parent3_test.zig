const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_nav_parent3 = @import("../formats/nav_parent3.zig");

test "NavParent3" {
    const file = try _imp_std.fs.cwd().openFile("../../src/nav_parent2.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_nav_parent3.NavParent3.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(8, r.ofs_tags);
    try _imp_std.testing.expectEqual(2, r.num_tags);
    try _imp_std.testing.expectEqualStrings("RAHC", (try r.tags()).items[0].name);
    try _imp_std.testing.expectEqual(32, (try r.tags()).items[0].ofs);
    try _imp_std.testing.expectEqual(3, (try r.tags()).items[0].num_items);
    try _imp_std.testing.expectEqualStrings("foo", (try (try r.tags()).items[0].tagContent()).?.content);
    try _imp_std.testing.expectEqualStrings("RAHC", (try r.tags()).items[1].name);
    try _imp_std.testing.expectEqual(35, (try r.tags()).items[1].ofs);
    try _imp_std.testing.expectEqual(6, (try r.tags()).items[1].num_items);
    try _imp_std.testing.expectEqualStrings("barbaz", (try (try r.tags()).items[1].tagContent()).?.content);
}
