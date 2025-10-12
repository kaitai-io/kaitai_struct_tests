const std = @import("std");
const imported_with_type_mismatch = @import("imported_with_type_mismatch.zig");

pub const FormatImportsWithTypeMismatch = struct {
    imp: imported_with_type_mismatch.ImportedWithTypeMismatch = undefined,

    pub fn create() FormatImportsWithTypeMismatch {
        var self = FormatImportsWithTypeMismatch{};
        self._read();
        return self;
    }

    fn _read(self: *FormatImportsWithTypeMismatch) void {
        self.imp = imported_with_type_mismatch.ImportedWithTypeMismatch.create();
    }
};
