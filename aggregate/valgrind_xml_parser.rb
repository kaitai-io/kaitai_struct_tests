# frozen_string_literal: true

require_relative 'test_parser'

require 'rexml/document'

# See the documentation of Valgrind's XML output at
# https://sourceware.org/git/?p=valgrind.git;a=blob;f=docs/internals/xml-output-protocol4.txt;hb=0b557127300197e0c779369d2e173eb85121fd66
class ValgrindXMLParser < TestParser
  def initialize(fn)
    @fn = fn
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.root.elements.each('error') do |err|
      test_name = stack_to_test_name(err.elements['stack'])
      unless test_name
        warn "[ValgrindXMLParser] warning: no test seems to be responsible for #{err.xpath} in #{@fn}"
        next
      end

      err_kind = err.elements['kind'].text
      status =
        if err_kind.start_with?('Leak_')
          :leak
        else
          :failed
        end
      msg =
        if err.elements['xwhat']
          err.elements['xwhat'].elements['text'].text
        elsif err.elements['what']
          err.elements['what'].text
        end
      failure = TestResult::Failure.new(nil, nil, msg, nil)
      tr = TestResult.new(underscore_to_ucamelcase(test_name), status, nil, failure)
      yield tr
    end
  end

  private

  ##
  # Given a `<stack>` element, returns the name of the corresponding test if
  # available, otherwise returns `nil`.
  #
  # @param stack [REXML::Element] "stack" element
  # @return [String, nil] test name or `nil`
  def stack_to_test_name(stack)
    stack.elements.reverse_each('frame') do |frame|
      path = frame_to_path(frame)
      next unless path

      next unless path.include?('/spec/cpp_stl')

      basename = File.basename(path, '.cpp')
      test_name = basename.delete_prefix('test_')
      return test_name
    end

    nil
  end

  def frame_to_path(frame)
    dir = frame.elements['dir']
    return nil unless dir

    file = frame.elements['file']
    return nil unless file

    File.join(dir.text, file.text)
  end
end
