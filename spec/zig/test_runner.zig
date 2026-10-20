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

pub fn main(init: std.process.Init) !void {
    const allocator = init.arena.allocator();
    const io = init.io;

    var args = try init.minimal.args.iterateAllocator(allocator);
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
        try tests.append(allocator, try runTest(allocator, init.minimal, test_fn, debug_info));
    }

    const report = Report{
        .zig_version = builtin.zig_version_string,
        .tests = tests.items,
    };

    const output_file = try std.Io.Dir.cwd().createFile(io, output_file_path, .{});
    defer output_file.close(io);

    var output_buffer: [1024]u8 = undefined;
    var output_file_writer = output_file.writer(io, &output_buffer);
    const writer = &output_file_writer.interface;

    try std.json.fmt(report, .{ .whitespace = .indent_2 }).format(writer);
    try writer.writeByte('\n');
    try writer.flush();
}

fn runTest(allocator: std.mem.Allocator, minimal: std.process.Init.Minimal, test_fn: TestFn, debug_info: *std.debug.SelfInfo) !TestResult {
    const test_fn_addr = @intFromPtr(test_fn.func);
    var symbols: std.ArrayList(std.debug.Symbol) = .empty;
    try debug_info.getSymbols(std.Options.debug_io, allocator, allocator, test_fn_addr, false, &symbols);
    const src_loc = symbols.items[0].source_location orelse unreachable;

    // Like the default test runner in Zig 0.16.0
    // (https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/lib/compiler/test_runner.zig#L275-L278),
    // provide a fresh `std.testing.io` for each test
    std.testing.io_instance = .init(std.testing.allocator, .{
        .argv0 = .init(minimal.args),
        .environ = minimal.environ,
    });
    defer std.testing.io_instance.deinit();
    std.testing.environ = minimal.environ;

    const result = test_fn.func();

    const error_obj: ?Error = if (result) |_| null else |err| error_blk: {
        const error_type = @errorName(err);
        const error_trace = if (@errorReturnTrace()) |trace| trace_blk: {
            var allocating_writer = std.Io.Writer.Allocating.init(allocator);
            defer allocating_writer.deinit();
            const writer = &allocating_writer.writer;
            try std.debug.writeErrorReturnTrace(trace, .{ .writer = writer, .mode = .no_color });
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
