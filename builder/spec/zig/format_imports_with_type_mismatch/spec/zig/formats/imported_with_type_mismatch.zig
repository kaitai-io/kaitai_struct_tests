pub const ImportedWithTypeMismatch = struct {
    one: u8 = undefined,

    pub fn create() ImportedWithTypeMismatch {
        var self = ImportedWithTypeMismatch{};
        self._read();
        return self;
    }

    fn _read(self: *ImportedWithTypeMismatch) void {
        self.one = -1;
    }
};
