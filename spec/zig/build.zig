const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const no_bin = b.option(bool, "no-bin", "skip emitting binary") orelse false;

    const kaitai_struct = b.dependency("kaitai_struct", .{
        .target = target,
    });

    const mod_kaitai_struct = kaitai_struct.module("kaitai_struct");

    // Create a test executable.
    const tests_exe = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("tests.zig"),
            .imports = &.{
                .{ .name = "kaitai_struct", .module = mod_kaitai_struct },
            },

            .target = target,
            .optimize = optimize,
        }),
        .test_runner = .{
            .path = b.path("test_runner.zig"),
            .mode = .simple,
        },
    });

    // A run step that will run the test executable.
    const run_tests_exe = b.addRunArtifact(tests_exe);
    const tests_report = run_tests_exe.addPrefixedOutputFileArg("--output-file=", "report.json");
    const tests_stderr = run_tests_exe.captureStdErr();

    // Copy the test artifacts to the "installation prefix" directory, which is
    // called `zig-out` by default, but can be changed using the `--prefix` flag
    // (`-p` for short).
    const install_tests_report = b.addInstallFile(tests_report, "report.json");
    const install_tests_stderr = b.addInstallFile(tests_stderr, "stderr.log");

    // A top level step for running all tests.
    const test_step = b.step("test", "Build and run tests");
    if (no_bin) {
        test_step.dependOn(&tests_exe.step);
    } else {
        test_step.dependOn(&run_tests_exe.step);
        test_step.dependOn(&install_tests_report.step);
        test_step.dependOn(&install_tests_stderr.step);
    }
}
