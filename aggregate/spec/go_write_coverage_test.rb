require 'minitest/autorun'
require 'tmpdir'
require 'json'
require 'open3'
require 'rexml/document'
require 'fileutils'

class GoWriteCoverageTest < Minitest::Test
  PRIMARY_EXPECTED_PARSE_SKIPS = %w[
    DefaultEndianExprException
    EofExceptionBitsBe
    EofExceptionBitsBe2
    EofExceptionBitsLe
    EofExceptionBitsLe2
    EofExceptionBytes
    EofExceptionSwitchUser
    EofExceptionU4
    EosExceptionBytes
    EosExceptionU4
    ValidFailAnyofInt
    ValidFailContents
    ValidFailEqBytes
    ValidFailEqInt
    ValidFailEqStr
    ValidFailExpr
    ValidFailInst
    ValidFailMaxInt
    ValidFailMinInt
    ValidFailRangeBytes
    ValidFailRangeFloat
    ValidFailRangeInt
    ValidFailRangeStr
  ].freeze

  def write_complete_corpus_report(dir, omitted_fixture: nil)
    manifest = JSON.parse(File.read(File.expand_path('../../spec/go/specwrite/coverage.json', __dir__)))
    corpus = manifest.fetch('$corpus')
    assert_equal PRIMARY_EXPECTED_PARSE_SKIPS, corpus.fetch('primary_expected_parse_skip')
    statuses = {}
    corpus.fetch('primary').each do |fixture|
      statuses[fixture] = PRIMARY_EXPECTED_PARSE_SKIPS.include?(fixture) ? 'skipped' : 'passed'
    end
    corpus.fetch('additional').each do |category, fixtures|
      next if category == 'shared_unsupported_generation'
      status = category == 'expected_parse_skip' ? 'skipped' : 'passed'
      fixtures.each { |fixture| statuses[fixture] = status }
    end
    statuses.delete(omitted_fixture)
    cases = statuses.map do |fixture, status|
      skipped = status == 'skipped' ? '<skipped message="expected parse failure"/>' : ''
      "<testcase name=\"Test#{fixture}\" time=\"0\">#{skipped}</testcase>"
    end
    File.write(File.join(dir, 'report.xml'), "<testsuites><testsuite>#{cases.join}</testsuite></testsuites>")
    File.write(File.join(dir, 'kst_adoption.log'), '')
  end

  def write_executable(path, contents)
    File.write(path, contents)
    File.chmod(0755, path)
  end

  def test_manual_negative_categories_remain_visible_without_becoming_roundtrips_or_skips
    Dir.mktmpdir do |dir|
      names = %w[CastToTop DebugArrayUserCurrentExcluded StrEncodingsEscapingEnc StrEncodingsEscapingEnc/negative_generation/read/str1]
      File.write(File.join(dir, 'report.xml'), '<testsuites><testsuite>' + names.map { |n| "<testcase name=\"Test#{n}\" time=\"0\"/>" }.join + '</testsuite></testsuites>')
      File.write(File.join(dir, 'kst_adoption.log'), '')
      converter = File.expand_path('../convert_to_json', __dir__)
      output, status = Open3.capture2e({'GO_WRITE_COVERAGE_MODE' => 'partial'}, converter, 'go-write', dir, File.join(dir, 'ci.json'))
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
      _, status = Open3.capture2e({'GO_WRITE_COVERAGE_MODE' => 'partial'}, converter, 'go-write', dir, File.join(dir, 'ci.json'))
      assert status.success?
      xml = REXML::Document.new(File.read(File.join(dir, 'report.xml')))
      assert_equal 1, xml.get_elements('//testcase[@name="TestDebugArrayUserCurrentExcluded"]/properties/property[@name="coverage.category"]').size
    end
  end

  def test_corpus_counts_fixtures_once_across_generated_and_manual_contracts
    Dir.mktmpdir do |dir|
      cases = [
        '<testcase name="TestEofExceptionBytes" time="0"><skipped message="expected parse failure"/></testcase>',
        '<testcase name="TestEofExceptionBytesWriteCapacity" time="0"/>',
        '<testcase name="TestExprIoTernary" time="0"/>',
        '<testcase name="TestExprIoTernaryBoundary" time="0"/>',
        '<testcase name="TestOpaqueWithParam" time="0"/>',
      ]
      File.write(File.join(dir, 'report.xml'), "<testsuites><testsuite>#{cases.join}</testsuite></testsuites>")
      File.write(File.join(dir, 'kst_adoption.log'), '')
      converter = File.expand_path('../convert_to_json', __dir__)
      output, status = Open3.capture2e({'GO_WRITE_COVERAGE_MODE' => 'partial'}, converter, 'go-write', dir, File.join(dir, 'ci.json'))
      assert status.success?, output

      result = JSON.parse(File.read(File.join(dir, 'ci.json')))
      corpus = result.fetch('$go_write_corpus')
      assert_equal({'missing' => 285, 'passed' => 1, 'skipped' => 1}, corpus.fetch('primary'))
      assert_equal({'missing' => 3, 'passed' => 1}, corpus.fetch('additional').fetch('manual_boundary'))
      assert_equal({'not_generated' => 5}, corpus.fetch('additional').fetch('shared_unsupported_generation'))
      assert_includes output, '"skipped":1'
      assert_includes output, '"manual_boundary":{"missing":3,"passed":1}'
    end
  end

  def test_production_mode_fails_closed_when_corpus_fixture_is_missing
    Dir.mktmpdir do |dir|
      write_complete_corpus_report(dir, omitted_fixture: 'BcdUserTypeBe')
      converter = File.expand_path('../convert_to_json', __dir__)
      output, status = Open3.capture2e(converter, 'go-write', dir, File.join(dir, 'ci.json'))

      refute status.success?, output
      assert_includes output, 'Go writer corpus validation failed'
      assert_includes output, 'BcdUserTypeBe: expected passed, got missing'
      assert_equal 1, output.scan(/: expected (?:passed|skipped), got missing/).size, output
    end
  end

  def test_production_wrapper_forces_strict_mode_over_ambient_partial
    Dir.mktmpdir do |dir|
      root = File.expand_path('../..', __dir__)
      FileUtils.cp(File.join(root, 'run-go-write'), dir)
      File.write(File.join(dir, 'config'), "TEST_OUT_DIR=test_out\ncd \"$(dirname \"$0\")\"\n")
      write_executable(File.join(dir, 'prepare-go'), "#!/bin/sh\nexit 0\n")
      write_executable(File.join(dir, 'kst-adoption-report'), "#!/bin/sh\n: > test_out/go-write/kst_adoption.log\n")

      FileUtils.cp_r(File.join(root, 'aggregate'), dir)
      spec_dir = File.join(dir, 'spec/go/specwrite')
      FileUtils.mkdir_p(spec_dir)
      FileUtils.cp(File.join(root, 'spec/go/specwrite/coverage.json'), spec_dir)

      bin_dir = File.join(dir, 'bin')
      FileUtils.mkdir_p(bin_dir)
      write_executable(File.join(bin_dir, 'go'), "#!/bin/sh\nprintf 'PASS\\n'\n")
      write_executable(File.join(bin_dir, 'go-junit-report'), <<~'RUBY')
        #!/usr/bin/env ruby
        output = ARGV.fetch(ARGV.index('-out') + 1)
        File.write(output, '<testsuites><testsuite><testcase name="TestBcdUserTypeBe" time="0"/></testsuite></testsuites>')
      RUBY

      env = {
        'GO_WRITE_COVERAGE_MODE' => 'partial',
        'PATH' => "#{bin_dir}:#{ENV.fetch('PATH')}",
      }
      output, status = Open3.capture2e(env, File.join(dir, 'run-go-write'), chdir: dir)

      refute status.success?, output
      assert_includes output, 'Go writer corpus validation failed'
      assert_includes output, 'BcdUserTypeLe: expected passed, got missing'
    end
  end
end
