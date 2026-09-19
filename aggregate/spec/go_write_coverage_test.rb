require 'minitest/autorun'
require 'tmpdir'
require 'json'
require 'open3'
require 'rexml/document'

class GoWriteCoverageTest < Minitest::Test
  def test_manual_negative_categories_remain_visible_without_becoming_roundtrips_or_skips
    Dir.mktmpdir do |dir|
      names = %w[CastToTop DebugArrayUserCurrentExcluded StrEncodingsEscapingEnc StrEncodingsEscapingEnc/negative_generation/read/str1]
      File.write(File.join(dir, 'report.xml'), '<testsuites><testsuite>' + names.map { |n| "<testcase name=\"Test#{n}\" time=\"0\"/>" }.join + '</testsuite></testsuites>')
      File.write(File.join(dir, 'kst_adoption.log'), '')
      converter = File.expand_path('../convert_to_json', __dir__)
      output, status = Open3.capture2e(converter, 'go-write', dir, File.join(dir, 'ci.json'))
      assert status.success?, output
      result = JSON.parse(File.read(File.join(dir, 'ci.json')))
      negative = result.fetch('DebugArrayUserCurrentExcluded')
      assert_equal 'passed', negative.fetch('status')
      assert_equal 'unsupported_serialization_semantics', negative.fetch('coverage').fetch('category')
      assert_equal false, negative.fetch('coverage').fetch('reference_writer_parity')
      assert_equal 'negative_generation', result.fetch(names.last).fetch('coverage').fetch('category')
      assert_includes output, '"unsupported_serialization_semantics":{"passed":1}'
      assert_includes output, '"negative_generation":{"passed":1}'
      xml = REXML::Document.new(File.read(File.join(dir, 'report.xml')))
      property = xml.elements['//testcase[@name="TestDebugArrayUserCurrentExcluded"]/properties/property[@name="coverage.category"]']
      refute_nil property
      assert_equal 'unsupported_serialization_semantics', property.attributes['value']
      # Repeated conversion must not duplicate XML metadata or alter counts.
      _, status = Open3.capture2e(converter, 'go-write', dir, File.join(dir, 'ci.json'))
      assert status.success?
      xml = REXML::Document.new(File.read(File.join(dir, 'report.xml')))
      assert_equal 1, xml.get_elements('//testcase[@name="TestDebugArrayUserCurrentExcluded"]/properties/property[@name="coverage.category"]').size
    end
  end
end
