require_relative 'junit_xml_parser'

# go-junit-report represents compilation/panic failures before tests execute as
# a package-level testcase named Failure. Keep this adaptation write-specific.
class GoWriteJUnitXMLParser < JUnitXMLParser
  def initialize(fn)
    super(fn)
    @docs.each do |doc|
      doc.elements.each('//testcase') do |tc|
        if ['', 'Failure'].include?(tc.attributes['name']) && (tc.elements['error'] || tc.elements['failure'])
          tc.attributes['name'] = "TestGoWritePackage/#{tc.attributes['classname']}"
        end
      end
    end
  end
end
