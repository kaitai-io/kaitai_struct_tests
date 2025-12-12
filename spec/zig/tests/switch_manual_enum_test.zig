const _imp_std = @import("std");
const _imp_kaitai_struct = @import("kaitai_struct");
const _imp_switch_manual_enum = @import("../formats/switch_manual_enum.zig");

test "SwitchManualEnum" {
    const file = try _imp_std.fs.cwd().openFile("../../src/switch_opcodes.bin", .{});
    defer file.close();
    var buffer: [8]u8 = undefined;
    var reader = file.reader(&buffer);
    const allocator = _imp_std.testing.allocator;
    var arena = _imp_std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);
    const r = try _imp_switch_manual_enum.SwitchManualEnum.create(&arena, &_io, null, null);
    try _imp_std.testing.expectEqual(4, r.opcodes.items.len);
    try _imp_std.testing.expectEqual(_imp_switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.strval, r.opcodes.items[0].code);
    try _imp_std.testing.expectEqualStrings("foobar", r.opcodes.items[0].body.?.strval.value);
    try _imp_std.testing.expectEqual(_imp_switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.intval, r.opcodes.items[1].code);
    try _imp_std.testing.expectEqual(66, r.opcodes.items[1].body.?.intval.value);
    try _imp_std.testing.expectEqual(_imp_switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.intval, r.opcodes.items[2].code);
    try _imp_std.testing.expectEqual(55, r.opcodes.items[2].body.?.intval.value);
    try _imp_std.testing.expectEqual(_imp_switch_manual_enum.SwitchManualEnum.Opcode.CodeEnum.strval, r.opcodes.items[3].code);
    try _imp_std.testing.expectEqualStrings("", r.opcodes.items[3].body.?.strval.value);
}
