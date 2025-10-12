pub const ImportedWithSyntaxError = struct {
    pub fn create() ImportedWithSyntaxError {
        var self = ImportedWithSyntaxError{};
        self._read();
        return self;
    }

    fn _read(self: *ImportedWithSyntaxError) void {
        _ = new byte[] { 80, 65, 67, 75, 45, 49 };
    }
};
