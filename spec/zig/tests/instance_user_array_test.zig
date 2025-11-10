const std = @import("std");
const kaitai_struct = @import("kaitai_struct");
const instance_user_array = @import("../formats/instance_user_array.zig");

test "InstanceUserArray" {
    const file = try std.fs.cwd().openFile("../../src/instance_std_array.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try instance_user_array.InstanceUserArray.create(&arena, &_io, null, null);
    try std.testing.expectEqual(16, r.ofs);
    try std.testing.expectEqual(3, r.qty_entries);
    try std.testing.expectEqual(4, r.entry_size);
    const user_entries = (try r.userEntries()).?;
    try std.testing.expectEqual(3, user_entries.items.len);
    try std.testing.expectEqual(4369, user_entries.items[0].word1);
    try std.testing.expectEqual(4369, user_entries.items[0].word2);
    try std.testing.expectEqual(8738, user_entries.items[1].word1);
    try std.testing.expectEqual(8738, user_entries.items[1].word2);
    try std.testing.expectEqual(13107, user_entries.items[2].word1);
    try std.testing.expectEqual(13107, user_entries.items[2].word2);
}
