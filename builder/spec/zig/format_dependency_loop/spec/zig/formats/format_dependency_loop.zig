pub const FormatDependencyLoop = struct {
    pub fn create() !FormatDependencyLoop {
        var self = FormatDependencyLoop{};
        try self._read();
        return self;
    }

    fn _read(self: *FormatDependencyLoop) !void {
        _ = self;
        _ = try FormatDependencyLoop.create();
    }
};
