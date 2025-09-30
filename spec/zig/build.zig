const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const kaitai_struct = b.dependency("kaitai_struct", .{
        .target = target,
    });

    const mod_kaitai_struct = kaitai_struct.module("kaitai_struct");

    // Create a test executable.
    const mod_tests = b.addTest(.{
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
    const run_mod_tests = b.addRunArtifact(mod_tests);
    const tests_report = run_mod_tests.addPrefixedOutputFileArg("--output-file=", "report.json");
    const tests_stderr = run_mod_tests.captureStdErr();

    // Copy the test artifacts to the "installation prefix" directory, which is
    // called `zig-out` by default, but can be changed using the `--prefix` flag
    // (`-p` for short).
    const install_tests_report = b.addInstallFile(tests_report, "report.json");
    const install_tests_stderr = b.addInstallFile(tests_stderr, "stderr.log");

    // A top level step for running all tests.
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&install_tests_report.step);
    test_step.dependOn(&install_tests_stderr.step);
}
