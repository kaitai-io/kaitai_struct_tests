pub const ImportedWithError = struct {
    one: u8 = undefined,

    pub fn create() ImportedWithError {
        var self = ImportedWithError{};
        self._read();
        return self;
    }

    fn _read(self: *ImportedWithError) void {
        self.one = -1;
    }
};
