# frozen_string_literal: true

require_relative 'test_parser'

require 'rexml/document'

# See the documentation of Valgrind's XML output at
# https://sourceware.org/git/?p=valgrind.git;a=blob;f=docs/internals/xml-output-protocol4.txt;hb=0b557127300197e0c779369d2e173eb85121fd66
class ValgrindXMLParser < TestParser
  def initialize(fn, glob = false)
    files = glob ? Dir.glob(fn) : [fn]
    @fn_doc_pairs = files.map { |f| [f, REXML::Document.new(File.read(f))] }
  end

  def each_test
    @fn_doc_pairs.each do |(fn, doc)|
      doc.root.elements.each('error') do |err|
        stack = err.elements['stack']
        test_name = stack_to_test_name(stack)
        unless test_name
          warn "[ValgrindXMLParser] warning: no test seems to be responsible for #{err.xpath} in #{fn}"
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

        file_name = nil
        line_num = nil
        stack.elements.each('frame') do |frame|
          path = frame_to_path(frame)
          next unless path
          next unless path.include?('/compiled/cpp_stl') || path.include?('/spec/cpp_stl')

          file_name = path
          line_num = frame.elements['line'].text
          break
        end

        failure = TestResult::Failure.new(file_name, line_num, msg, nil)
        tr = TestResult.new(underscore_to_ucamelcase(test_name), status, nil, failure)
        yield tr
      end
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
      # # Since Ruby 2.5 (see
      # # https://www.bigbinary.com/blog/ruby-2-5-added-delete_prefix-and-delete_suffix-methods),
      # # we could use the `String#delete_prefix` method, but at the time of
      # # writing, some of our C++/STL targets still use older versions of Ruby,
      # # such as Ruby 1.9. Therefore, we use `String#sub` instead - see also
      # # <https://docs.rubocop.org/rubocop-performance/cops_performance.html#performancedeleteprefix>.
      # test_name = basename.delete_prefix('test_')
      test_name = basename.sub(/\Atest_/, '')
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
