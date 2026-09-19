require 'json'
require 'rexml/document'

# These are test contracts, not blanket fixture exclusions. A passing negative
# contract must not be interpreted as successful roundtrip coverage.
module GoWriteCoverage
  def self.annotate(tests, report_path)
    manifest = JSON.parse(File.read(File.expand_path('../spec/go/specwrite/coverage.json', __dir__)))
    summary = {}
    tests.each do |name, result|
      fixture = name.split('/').first
      coverage = manifest[fixture]
      next unless coverage
      result['coverage'] = coverage
      next unless name == fixture
      counts = (summary[coverage.fetch('category')] ||= {})
      status = result.fetch('status', 'unknown')
      counts[status] = (counts[status] || 0) + 1
    end

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
    puts "COVERAGE (classified fixtures, not roundtrip totals): #{JSON.generate(summary)}"
    tests
  end
end
