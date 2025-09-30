const std = @import("std");
const builtin = @import("builtin");
const TestFn = std.builtin.TestFn;

const Report = struct {
    zig_version: []const u8,
    tests: []const TestResult,
};

const TestResult = struct {
    name: []const u8,
    file: []const u8,
    line: u64,
    @"error": ?Error,
};

const Error = struct {
    type: []const u8,
    trace: []const u8,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // Skip the executable name
    std.debug.assert(args.skip());

    var output_file_path_optional: ?[]const u8 = null;

    while (args.next()) |arg| {
        const OUTPUT_FILE_OPTION_NAME = "--output-file=";
        if (std.mem.startsWith(u8, arg, OUTPUT_FILE_OPTION_NAME)) {
            output_file_path_optional = arg[OUTPUT_FILE_OPTION_NAME.len..];
        } else {
            std.debug.print("Error: unexpected argument '{s}'\n", .{arg});
            std.process.exit(1);
        }
    }

    const output_file_path = output_file_path_optional orelse {
        std.debug.print("Error: missing mandatory --output-file=... option", .{});
        std.process.exit(1);
    };

    const test_fns: []const TestFn = builtin.test_functions;
    const debug_info = try std.debug.getSelfDebugInfo();

    var tests: std.ArrayList(TestResult) = try .initCapacity(allocator, test_fns.len);

    for (test_fns) |test_fn| {
        // Use a unique prefix to prevent any other line printed to stderr from
        // being mistaken for this test start marker
        std.debug.print("=@= ZIG_TEST_START {s} ~#~\n", .{test_fn.name});
        try tests.append(allocator, try runTest(allocator, test_fn, debug_info));
    }

    const report = Report{
        .zig_version = builtin.zig_version_string,
        .tests = tests.items,
    };

    const output_file = try std.fs.cwd().createFile(output_file_path, .{});
    defer output_file.close();

    var output_buffer: [1024]u8 = undefined;
    var output_file_writer = output_file.writer(&output_buffer);
    const writer = &output_file_writer.interface;

    try std.json.fmt(report, .{ .whitespace = .indent_2 }).format(writer);
    try writer.writeByte('\n');
    try writer.flush();
}

fn runTest(allocator: std.mem.Allocator, test_fn: TestFn, debug_info: *std.debug.SelfInfo) !TestResult {
    const test_fn_addr = @intFromPtr(test_fn.func);
    const module: *std.debug.SelfInfo.Module = try debug_info.getModuleForAddress(test_fn_addr);
    const symbol_info = try module.getSymbolAtAddress(debug_info.allocator, test_fn_addr);
    const src_loc = symbol_info.source_location orelse unreachable;

    const result = test_fn.func();

    const error_obj: ?Error = if (result) |_| null else |err| error_blk: {
        const error_type = @errorName(err);
        const error_trace = if (@errorReturnTrace()) |trace| trace_blk: {
            var allocating_writer = std.Io.Writer.Allocating.init(allocator);
            defer allocating_writer.deinit();
            const writer = &allocating_writer.writer;
            try std.debug.writeStackTrace(trace.*, writer, debug_info, .no_color);
            break :trace_blk try allocating_writer.toOwnedSlice();
        } else unreachable;
        break :error_blk .{
            .type = error_type,
            .trace = error_trace,
        };
    };

    return .{
        .name = test_fn.name,
        .file = src_loc.file_name,
        .line = src_loc.line,
        .@"error" = error_obj,
    };
}
