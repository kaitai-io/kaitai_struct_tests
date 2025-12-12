const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_enum_if = @import("../formats/enum_if.zig");

test "EnumIf" {
    const file = try _imp_std.fs.cwd().openFile("../../src/if_struct.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_enum_if.EnumIf.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(_imp_enum_if.EnumIf.Opcodes.a_string, r.op1.opcode);
    try _imp_std.testing.expectEqualStrings("foo", r.op1.arg_str.?.str);
    try _imp_std.testing.expectEqual(_imp_enum_if.EnumIf.Opcodes.a_tuple, r.op2.opcode);
    try _imp_std.testing.expectEqual(66, r.op2.arg_tuple.?.num1);
    try _imp_std.testing.expectEqual(67, r.op2.arg_tuple.?.num2);
    try _imp_std.testing.expectEqual(_imp_enum_if.EnumIf.Opcodes.a_string, r.op3.opcode);
    try _imp_std.testing.expectEqualStrings("bar", r.op3.arg_str.?.str);
}
