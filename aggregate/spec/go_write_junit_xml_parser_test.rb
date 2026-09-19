require 'minitest/autorun'
require 'tmpdir'
require 'json'
require 'open3'
require_relative '../go_write_junit_xml_parser'

class GoWriteJUnitXMLParserTest < Minitest::Test
  def test_setup_failure_replaces_stale_success_json_with_package_diagnostic
    Dir.mktmpdir do |dir|
      File.write(File.join(dir, 'report.xml'), <<~XML)
        <testsuites><testsuite name="kaitai/specwrite" tests="1" errors="1">
          <testcase name="[setup failed]" classname="kaitai/specwrite" time="0">
            <error message="Build error">process_custom.go:8:2: package nested/deeply is not in std</error>
          </testcase>
        </testsuite></testsuites>
      XML
      File.write(File.join(dir, 'kst_adoption.log'), '')
      json_path = File.join(dir, 'ci.json')
      File.write(json_path, '{"StaleSuccess":{"status":"passed"}}')
      converter = File.expand_path('../convert_to_json', __dir__)
      output, status = Open3.capture2e(converter, 'go-write', dir, json_path)
      assert status.success?, output
      result = JSON.parse(File.read(json_path))
      refute result.key?('StaleSuccess')
      package = result.fetch('GoWritePackage/kaitai/specwrite')
      assert_equal 'failed', package.fetch('status')
      assert_equal 'Build error', package.fetch('failure').fetch('message')
      assert_equal 'process_custom.go:8:2: package nested/deeply is not in std', package.fetch('failure').fetch('trace')
      assert_includes output, '"failed":1'
    end
  end

  def test_package_failure_is_explicit_and_read_parser_is_unchanged
    Dir.mktmpdir do |dir|
      path = File.join(dir, 'report.xml')
      File.write(path, '<testsuites><testsuite><testcase name="Failure" classname="kaitai/specwrite" time="0"><error message="Build failed">undefined: badSymbol</error></testcase></testsuite></testsuites>')
      result = GoWriteJUnitXMLParser.new(path).to_enum(:each_test).to_a
      assert_equal 1, result.size
      assert_equal 'GoWritePackage/kaitai/specwrite', result[0].name
      assert_equal :failed, result[0].status
      assert_equal 'undefined: badSymbol', result[0].failure.trace
      assert_raises(RuntimeError) { JUnitXMLParser.new(path).to_enum(:each_test).to_a }
    end
  end

  def test_unnamed_dependency_build_failure_is_explicit
    Dir.mktmpdir do |dir|
      path = File.join(dir, 'report.xml')
      File.write(path, '<testsuites><testsuite><testcase name="" classname="test_formats_write" time="0"><error message="Build error">undefined: badSymbol</error></testcase></testsuite></testsuites>')
      result = GoWriteJUnitXMLParser.new(path).to_enum(:each_test).to_a
      assert_equal 'GoWritePackage/test_formats_write', result[0].name
      assert_equal :failed, result[0].status
      assert_equal 'undefined: badSymbol', result[0].failure.trace
    end
  end

  def test_regular_test_names_are_preserved
    Dir.mktmpdir do |dir|
      path = File.join(dir, 'report.xml')
      File.write(path, '<testsuites><testsuite><testcase name="TestFixedContents" time="0" /></testsuite></testsuites>')
      result = GoWriteJUnitXMLParser.new(path).to_enum(:each_test).to_a
      assert_equal 'FixedContents', result[0].name
      assert_equal :passed, result[0].status
    end
  end
end
