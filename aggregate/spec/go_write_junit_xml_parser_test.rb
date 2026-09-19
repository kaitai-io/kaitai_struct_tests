require 'minitest/autorun'
require 'tmpdir'
require_relative '../go_write_junit_xml_parser'

class GoWriteJUnitXMLParserTest < Minitest::Test
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
