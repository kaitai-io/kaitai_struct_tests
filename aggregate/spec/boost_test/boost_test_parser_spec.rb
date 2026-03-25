# frozen_string_literal: true

require_relative '../../boost_test_parser'
require 'rspec' # normally not needed, but RubyMine doesn't autocomplete RSpec methods without it

RSpec.describe BoostTestParser do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  # The log comes from https://github.com/kaitai-io/ci_artifacts/blob/8b27b4eeaa8653cd33940fb620c184cd9763e6f0/test_out/cpp_stl_11/results.xml
  context 'aborted_gcc' do
    before :context do
      Dir.chdir("#{@spec_dir}/aborted_gcc")
    end

    describe '#aborted_tests' do
      it 'parses aborted tests' do
        infile = 'test_out/cpp_stl_11'
        p = BoostTestParser.new("#{infile}/results.xml")
        expect(p.aborted_tests).to eq [
          'test_debug_array_user',
        ]
      end
    end
  end

  # The log comes from https://github.com/kaitai-io/ci_artifacts/blob/a588faa1a35b658116d4ab5551f0863e61a7324c/test_out/cpp_stl_11/results.xml
  context 'aborted_msvc' do
    before :context do
      Dir.chdir("#{@spec_dir}/aborted_msvc")
    end

    describe '#aborted_tests' do
      it 'parses aborted tests' do
        infile = 'test_out/cpp_stl_11'
        p = BoostTestParser.new("#{infile}/results.xml")
        expect(p.aborted_tests).to eq [
          'test_debug_array_user',
        ]
      end
    end
  end

  # This test covers a tricky case where the XML log is pretty-printed by
  # CppBuilder (see
  # https://github.com/kaitai-io/kaitai_struct_tests/commit/7b4e9d60700a27f25239fb0b3149728a04bd160b)
  # and the exception message is split into two CDATA sections (this is forced
  # by the presence of the sequence `]]>`, which cannot be represented in a
  # single CDATA section, see https://en.wikipedia.org/wiki/CDATA#Nesting).
  #
  # For BoostTestParser to pass this test, it must concatenate all CDATA
  # sections to form the complete message, while ignoring regular text nodes
  # containing only whitespace that were inserted during pretty-printing.
  context 'exception_in_pretty_xml' do
    before :context do
      Dir.chdir("#{@spec_dir}/exception_in_pretty_xml")
    end

    describe '#each_test' do
      it 'parses the message from <Exception> correctly' do
        infile = 'test_out/cpp_stl_11'
        p = BoostTestParser.new("#{infile}/results-1.xml")
        expect(p.to_enum(:each_test).map { |t| [t.name, t.to_h] }).to eq [
          [
            'HelloWorld',
            {
              'status' => :failed,
              'elapsed' => 0.0,
              'failure' => {
                'file_name' => 'unknown location',
                'line_num' => '0',
                'message' => 'std::runtime_error: special sequence ]]> in a CDATA section',
                'trace' => nil
              }
            }
          ],
        ]
      end
    end
  end
end
