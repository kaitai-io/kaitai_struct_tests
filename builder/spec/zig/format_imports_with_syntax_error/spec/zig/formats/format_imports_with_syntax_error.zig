const std = @import("std");
const imported_with_syntax_error = @import("imported_with_syntax_error.zig");

pub const FormatImportsWithSyntaxError = struct {
    imp: imported_with_syntax_error.ImportedWithSyntaxError = undefined,

    pub fn create() FormatImportsWithSyntaxError {
        var self = FormatImportsWithSyntaxError{};
        self._read();
        return self;
    }

    fn _read(self: *FormatImportsWithSyntaxError) void {
        self.imp = imported_with_syntax_error.ImportedWithSyntaxError.create();
    }
};
