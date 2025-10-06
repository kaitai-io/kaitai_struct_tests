const std = @import("std");
const imported_with_error = @import("imported_with_error.zig");

pub const FormatImportsWithError = struct {
    imp: imported_with_error.ImportedWithError = undefined,

    pub fn create() FormatImportsWithError {
        var self = FormatImportsWithError{};
        self._read();
        return self;
    }

    fn _read(self: *FormatImportsWithError) void {
        self.imp = imported_with_error.ImportedWithError.create();
    }
};
