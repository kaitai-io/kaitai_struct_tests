require_relative 'junit_xml_parser'

# go-junit-report represents compilation/panic failures before tests execute as
# package-level testcases. Keep this adaptation write-specific.
class GoWriteJUnitXMLParser < JUnitXMLParser
  def initialize(fn)
    super(fn)
    @docs.each do |doc|
      doc.elements.each('//testcase') do |tc|
        if ['', 'Failure', '[setup failed]'].include?(tc.attributes['name']) && (tc.elements['error'] || tc.elements['failure'])
          tc.attributes['name'] = "TestGoWritePackage/#{tc.attributes['classname']}"
        end
      end
    end
  end
end
