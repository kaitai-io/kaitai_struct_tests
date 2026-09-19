require 'minitest/autorun'
require 'tmpdir'
require 'fileutils'
require 'open3'
require 'json'

# Run after generating eof_exception_switch_user and eos_exception_bytes in
# both spec/go/specwrite and compiled/go/src/test_formats_write. Exercise the
# actual generated assertions and parsers in an isolated copy of the harness.
class GoWriteEOFTest < Minitest::Test
  ROOT = File.expand_path('../..', __dir__)

  def setup
    @dir = Dir.mktmpdir('go-write-eof')
    @module_dir = File.join(@dir, 'spec/go/specwrite')
    FileUtils.mkdir_p(@module_dir)
    FileUtils.mkdir_p(File.join(@dir, 'src'))
    %w[common_spec.go eof_exception_switch_user_test.go eos_exception_bytes_test.go go.sum].each do |name|
      FileUtils.cp(File.join(ROOT, 'spec/go/specwrite', name), @module_dir)
    end
    mod = File.read(File.join(ROOT, 'spec/go/specwrite/go.mod'))
      .sub('../../../../runtime/go', File.expand_path('../runtime/go', ROOT))
      .sub('../../../compiled/go/src/test_formats_write', File.join(ROOT, 'compiled/go/src/test_formats_write'))
    File.write(File.join(@module_dir, 'go.mod'), mod)
  end

  def teardown
    FileUtils.remove_entry(@dir) if @dir
  end

  def run_generated(name)
    output, status = Open3.capture2e('go', 'test', '-json', '-count=1', '-run', "^#{name}$", '.', chdir: @module_dir)
    events = output.lines.map do |line|
      JSON.parse(line) rescue nil
    end.compact
    diagnostics = events.map { |e| e['Output'] }.compact.join
    [diagnostics.empty? ? output : diagnostics, status, events]
  end

  def test_zero_byte_eof_satisfies_expected_end_of_stream
    FileUtils.cp(File.join(ROOT, 'src/enum_negative.bin'), File.join(@dir, 'src'))
    output, status, events = run_generated('TestEofExceptionSwitchUser')
    assert status.success?, output
    assert events.any? { |e| e['Test'] == 'TestEofExceptionSwitchUser' && e['Action'] == 'skip' }, output
  end

  def test_partial_read_eof_satisfies_expected_end_of_stream
    FileUtils.cp(File.join(ROOT, 'src/term_strz.bin'), File.join(@dir, 'src'))
    output, status, events = run_generated('TestEosExceptionBytes')
    assert status.success?, output
    assert events.any? { |e| e['Test'] == 'TestEosExceptionBytes' && e['Action'] == 'skip' }, output
  end

  def test_non_eof_file_error_fails_with_diagnostic
    output, status, events = run_generated('TestEofExceptionSwitchUser')
    refute status.success?, output
    assert events.any? { |e| e['Test'] == 'TestEofExceptionSwitchUser' && e['Action'] == 'fail' }, output
    assert_includes output, 'expected'
    assert_includes output, 'enum_negative.bin: no such file or directory'
  end

  def test_successful_parse_does_not_satisfy_expected_end_of_stream
    File.binwrite(File.join(@dir, 'src/enum_negative.bin'), [2, 0, 7, 0].pack('C*'))
    output, status, events = run_generated('TestEofExceptionSwitchUser')
    refute status.success?, output
    assert events.any? { |e| e['Test'] == 'TestEofExceptionSwitchUser' && e['Action'] == 'fail' }, output
    assert_includes output, '<nil>'
  end
end
