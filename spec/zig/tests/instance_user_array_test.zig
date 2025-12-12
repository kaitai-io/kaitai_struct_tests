const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_instance_user_array = @import("../formats/instance_user_array.zig");

test "InstanceUserArray" {
    const file = try _imp_std.fs.cwd().openFile("../../src/instance_std_array.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_instance_user_array.InstanceUserArray.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(16, r.ofs);
    try _imp_std.testing.expectEqual(3, r.qty_entries);
    try _imp_std.testing.expectEqual(4, r.entry_size);
    const user_entries = (try r.userEntries()).?;
    try _imp_std.testing.expectEqual(3, user_entries.items.len);
    try _imp_std.testing.expectEqual(4369, user_entries.items[0].word1);
    try _imp_std.testing.expectEqual(4369, user_entries.items[0].word2);
    try _imp_std.testing.expectEqual(8738, user_entries.items[1].word1);
    try _imp_std.testing.expectEqual(8738, user_entries.items[1].word2);
    try _imp_std.testing.expectEqual(13107, user_entries.items[2].word1);
    try _imp_std.testing.expectEqual(13107, user_entries.items[2].word2);
}
