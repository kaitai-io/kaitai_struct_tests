pub const SpecTypeMismatchFormatParam = struct {
    pub fn create(_io: void) !*SpecTypeMismatchFormatParam {
        _ = _io;
        return error.EndOfStream;
    }
};
