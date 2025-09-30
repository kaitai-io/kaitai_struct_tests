const std = @import("std");
const kaitai_struct = @import("kaitai_struct");

pub const HelloWorld = struct {
    _arena: *std.heap.ArenaAllocator,
    _io: *kaitai_struct.KaitaiStream,
    _parent: ?*anyopaque,
    _root: ?*HelloWorld,
    one: u8 = undefined,

    pub fn create(_arena: *std.heap.ArenaAllocator, _io: *kaitai_struct.KaitaiStream, _parent: ?*anyopaque, _root: ?*HelloWorld) !*HelloWorld {
        const self = try _arena.allocator().create(HelloWorld);
        self.* = .{
            ._arena = _arena,
            ._io = _io,
            ._parent = _parent,
            ._root = _root orelse self,
        };
        try self._read();
        return self;
    }

    fn _allocator(self: *const HelloWorld) std.mem.Allocator {
        return self._arena.allocator();
    }

    fn _read(self: *HelloWorld) !void {
        self.one = try self._io.readU1();
    }
};
