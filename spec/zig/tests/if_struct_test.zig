const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_if_struct = @import("../formats/if_struct.zig");

test "IfStruct" {
    const file = try _imp_std.fs.cwd().openFile("../../src/if_struct.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_if_struct.IfStruct.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(83, r.op1.opcode);
    try _imp_std.testing.expectEqual(null, r.op1.arg_tuple);
    try _imp_std.testing.expectEqualStrings("foo", r.op1.arg_str.?.str);
    try _imp_std.testing.expectEqual(84, r.op2.opcode);
    try _imp_std.testing.expectEqual(66, r.op2.arg_tuple.?.num1);
    try _imp_std.testing.expectEqual(67, r.op2.arg_tuple.?.num2);
    try _imp_std.testing.expectEqual(null, r.op2.arg_str);
    try _imp_std.testing.expectEqual(83, r.op3.opcode);
    try _imp_std.testing.expectEqual(null, r.op3.arg_tuple);
    try _imp_std.testing.expectEqualStrings("bar", r.op3.arg_str.?.str);
}
