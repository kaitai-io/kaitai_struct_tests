require 'json'
require 'rexml/document'

# These are test contracts, not blanket fixture exclusions. A passing negative
# contract must not be interpreted as successful roundtrip coverage.
module GoWriteCoverage
  ADDITIONAL_EXPECTED_STATUS = {
    'automatic_roundtrip' => 'passed',
    'expected_parse_skip' => 'skipped',
    'manual_boundary' => 'passed',
    'negative_generation' => 'passed',
    'shared_unsupported_generation' => 'not_generated',
  }.freeze

  STATUS_PRIORITY = {
    'passed' => 0,
    'skipped' => 1,
    'failed' => 2,
    'spec_build_failed' => 3,
    'format_build_failed' => 4,
  }.freeze

  def self.merge_status(old_status, new_status)
    old_status = old_status.to_s unless old_status.nil?
    new_status = new_status.to_s unless new_status.nil?
    return new_status if old_status.nil?
    return old_status if new_status.nil?
    old_rank = STATUS_PRIORITY.fetch(old_status, STATUS_PRIORITY.size)
    new_rank = STATUS_PRIORITY.fetch(new_status, STATUS_PRIORITY.size)
    new_rank > old_rank ? new_status : old_status
  end

  def self.count_fixtures(fixtures, statuses, missing_status = 'missing')
    fixtures.each_with_object(Hash.new(0)) do |fixture, counts|
      counts[statuses.fetch(fixture, missing_status)] += 1
    end.sort.to_h
  end

  def self.expected_corpus_statuses(corpus)
    primary = corpus.fetch('primary')
    primary_skips = corpus.fetch('primary_expected_parse_skip')
    unknown_primary_skips = primary_skips - primary
    unless unknown_primary_skips.empty?
      raise "primary_expected_parse_skip contains non-primary fixtures: #{unknown_primary_skips.sort.join(', ')}"
    end

    statuses = primary.each_with_object({}) { |fixture, result| result[fixture] = 'passed' }
    primary_skips.each { |fixture| statuses[fixture] = 'skipped' }

    additional = corpus.fetch('additional')
    unexpected_categories = additional.keys - ADDITIONAL_EXPECTED_STATUS.keys
    missing_categories = ADDITIONAL_EXPECTED_STATUS.keys - additional.keys
    unless unexpected_categories.empty? && missing_categories.empty?
      raise "invalid additional corpus categories: missing=#{missing_categories.sort.join(',')} unexpected=#{unexpected_categories.sort.join(',')}"
    end
    additional.each do |category, fixtures|
      fixtures.each do |fixture|
        raise "duplicate corpus fixture: #{fixture}" if statuses.key?(fixture)
        statuses[fixture] = ADDITIONAL_EXPECTED_STATUS.fetch(category)
      end
    end
    statuses
  end

  def self.validate_corpus!(corpus, fixture_statuses, corpus_summary)
    expected_statuses = expected_corpus_statuses(corpus)
    errors = expected_statuses.each_with_object([]) do |(fixture, expected), result|
      actual = fixture_statuses.fetch(fixture, expected == 'not_generated' ? 'not_generated' : 'missing')
      result << "#{fixture}: expected #{expected}, got #{actual}" unless actual == expected
    end

    allowed_non_corpus = corpus.fetch('support_tests')
    unexpected_fixtures = fixture_statuses.keys - expected_statuses.keys - allowed_non_corpus
    errors << "unexpected corpus fixtures: #{unexpected_fixtures.sort.join(', ')}" unless unexpected_fixtures.empty?

    expected_summary = {
      'primary' => count_fixtures(corpus.fetch('primary'), expected_statuses),
      'additional' => {},
    }
    corpus.fetch('additional').each do |category, fixtures|
      expected_summary['additional'][category] = count_fixtures(fixtures, expected_statuses)
    end
    unless corpus_summary == expected_summary
      errors << "exact corpus counts differ: expected #{JSON.generate(expected_summary)}, got #{JSON.generate(corpus_summary)}"
    end

    raise "Go writer corpus validation failed:\n- #{errors.join("\n- ")}" unless errors.empty?
  end

  def self.annotate(tests, report_path, strict: true)
    manifest = JSON.parse(File.read(File.expand_path('../spec/go/specwrite/coverage.json', __dir__)))
    corpus = manifest.delete('$corpus')
    fixture_statuses = {}
    tests.each do |name, result|
      fixture = name.split('/').first
      coverage = manifest[fixture]
      result['coverage'] = coverage if coverage
      next unless name == fixture
      canonical_fixture = coverage && coverage['fixture']
      canonical_fixture = fixture if canonical_fixture.nil? && !(coverage && coverage['fixtures'])
      next unless canonical_fixture
      fixture_statuses[canonical_fixture] = merge_status(fixture_statuses[canonical_fixture], result['status'])
    end

    classified = {}
    manifest.each do |name, coverage|
      fixture = coverage['fixture']
      next unless fixture && fixture_statuses.key?(fixture)
      classified[fixture] ||= coverage
    end
    coverage_summary = {}
    classified.each do |fixture, coverage|
      counts = (coverage_summary[coverage.fetch('category')] ||= {})
      status = fixture_statuses.fetch(fixture)
      counts[status] = (counts[status] || 0) + 1
    end

    corpus_summary = {
      'primary' => count_fixtures(corpus.fetch('primary'), fixture_statuses),
      'additional' => {},
    }
    corpus.fetch('additional').each do |category, fixtures|
      missing_status = category == 'shared_unsupported_generation' ? 'not_generated' : 'missing'
      corpus_summary['additional'][category] = count_fixtures(fixtures, fixture_statuses, missing_status)
    end
    tests['$go_write_corpus'] = corpus_summary
    validate_corpus!(corpus, fixture_statuses, corpus_summary) if strict

    if File.exist?(report_path)
      doc = REXML::Document.new(File.read(report_path))
      doc.elements.each('//testcase') do |testcase|
        fixture = testcase.attributes['name'].to_s.sub(/^Test/, '').split('/').first
        coverage = manifest[fixture]
        next unless coverage
        properties = testcase.elements['properties'] || testcase.add_element('properties')
        coverage.each do |key, value|
          name = "coverage.#{key}"
          properties.get_elements('property').each { |p| properties.delete_element(p) if p.attributes['name'] == name }
          properties.add_element('property', {'name' => name, 'value' => value.to_s})
        end
      end
      File.write(report_path, doc.to_s)
    end
    puts "COVERAGE (classified fixtures, deduplicated across manual subtests): #{JSON.generate(coverage_summary)}"
    puts "CORPUS (fixture-level, raw Go tests reported separately): #{JSON.generate(corpus_summary)}"
    tests
  end
end
