const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_switch_manual_int_size = @import("../formats/switch_manual_int_size.zig");

test "SwitchManualIntSize" {
    const file = try _imp_std.fs.cwd().openFile("../../src/switch_tlv.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_switch_manual_int_size.SwitchManualIntSize.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(4, r.chunks.items.len);
    try _imp_std.testing.expectEqual(17, r.chunks.items[0].code);
    try _imp_std.testing.expectEqualStrings("Stuff", r.chunks.items[0].body.chunk_meta.title);
    try _imp_std.testing.expectEqualStrings("Me", r.chunks.items[0].body.chunk_meta.author);
    try _imp_std.testing.expectEqual(34, r.chunks.items[1].code);
    try _imp_std.testing.expectEqualDeep(@as([]const []const u8, &.{ "AAAA", "BBBB", "CCCC" }), r.chunks.items[1].body.chunk_dir.entries.items);
    try _imp_std.testing.expectEqual(51, r.chunks.items[2].code);
    try _imp_std.testing.expectEqualSlices(u8, &[_]u8{ 16, 32, 48, 64, 80, 96, 112, 128 }, r.chunks.items[2].body.bytes);
    try _imp_std.testing.expectEqual(255, r.chunks.items[3].code);
    try _imp_std.testing.expectEqualSlices(u8, &[_]u8{  }, r.chunks.items[3].body.bytes);
}
