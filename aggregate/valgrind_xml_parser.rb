# frozen_string_literal: true

require_relative 'test_parser'

require 'rexml/document'
require 'set'

# See the documentation of Valgrind's XML output at
# https://sourceware.org/git/?p=valgrind.git;a=blob;f=docs/internals/xml-output-protocol4.txt;hb=0b557127300197e0c779369d2e173eb85121fd66
class ValgrindXMLParser < TestParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.root.elements.each('error') { |err|
      affected = Set.new

      err.elements.each('stack') { |stack|
        stack.elements.each('frame') { |frame|
          path = frame_to_path(frame)
          next unless path

          kind = file_to_kind(path)
          next unless kind

          basename = File.basename(path, '.cpp')
          test_name = kind == :spec ? basename.delete_prefix('test_') : basename
          affected << test_name
        }
      }

      next if affected.empty?

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
      affected.each { |name|
        tr = TestResult.new(underscore_to_ucamelcase(name), status, nil, failure)
        yield tr
      }
    }
  end

  private

  def frame_to_path(frame)
    dir = frame.elements['dir']
    return nil unless dir

    file = frame.elements['file']
    return nil unless file

    File.join(dir.text, file.text)
  end

  def file_to_kind(path)
    if path.include?('/compiled/cpp_stl')
      :format
    elsif path.include?('/spec/cpp_stl')
      :spec
    end
  end
end
