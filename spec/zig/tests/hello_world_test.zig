const std = @import("std");
const testing = std.testing;
const kaitai_struct = @import("kaitai_struct");
const hello_world = @import("../formats/hello_world.zig");

test "HelloWorld" {
    const file = try std.fs.cwd().openFile("../../src/fixed_struct.bin", .{});
    defer file.close();
    var buffer: [4]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try hello_world.HelloWorld.create(&arena, &_io, null, null);
    try testing.expectEqual(81, r.one);
}
