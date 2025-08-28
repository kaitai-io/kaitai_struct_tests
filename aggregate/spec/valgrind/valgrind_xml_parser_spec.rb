# frozen_string_literal: true

require_relative '../../valgrind_xml_parser'
require 'rspec' # normally not needed, but RubyMine doesn't autocomplete RSpec methods without it

RSpec.describe ValgrindXMLParser do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  # The log comes from https://github.com/kaitai-io/ci_artifacts/blob/6b521282e8628272e72a9c9070b64879f23a8041/test_out/cpp_stl_11/valgrind-1.xml
  context 'invalid_read_clang11' do
    before :context do
      Dir.chdir("#{@spec_dir}/invalid_read_clang11")
    end

    describe '#each_test' do
      it 'parses Memcheck errors' do
        p = ValgrindXMLParser.new('test_out/cpp_stl_11/valgrind-1.xml')
        expect(p.to_enum(:each_test).map { |t| [t.name, t.to_h] }).to eq [
          [
            'DebugArrayUser',
            {
              'status' => :failed,
              'elapsed' => nil,
              'failure' => {
                'file_name' => '/tests/compiled/cpp_stl_11/debug_array_user.cpp',
                'line_num' => '37',
                'message' => 'Invalid read of size 8',
                'trace' => nil
              }
            }
          ],
          [
            'DebugArrayUser',
            {
              'status' => :leak,
              'elapsed' => nil,
              'failure' => {
                'file_name' => '/tests/spec/cpp_stl_11/test_debug_array_user.cpp',
                'line_num' => '8',
                'message' => '8,192 bytes in 1 blocks are definitely lost in loss record 7 of 7',
                'trace' => nil
              }
            }
          ]
        ]
      end
    end
  end

  # The log comes from https://github.com/kaitai-io/ci_artifacts/blob/cd0ed47c0ac3cd064cb8b87da03534d6ff005048/test_out/cpp_stl_11/valgrind.xml
  context 'uninit_clang3.5' do
    before :context do
      Dir.chdir("#{@spec_dir}/uninit_clang3.5")
    end

    describe '#each_test' do
      it 'parses Memcheck errors' do
        p = ValgrindXMLParser.new('test_out/cpp_stl_11/valgrind.xml')
        expect(p.to_enum(:each_test).map { |t| [t.name, t.to_h] }).to eq [
          [
            'ValidNotParsedIf',
            {
              'status' => :failed,
              'elapsed' => nil,
              'failure' => {
                'file_name' => '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/valid_not_parsed_if.cpp',
                'line_num' => '20',
                'message' => 'Conditional jump or move depends on uninitialised value(s)',
                'trace' => nil
              }
            }
          ],
          [
            'EosExceptionBytes',
            {
              'status' => :leak,
              'elapsed' => nil,
              'failure' => {
                'file_name' => '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/eos_exception_bytes.cpp',
                'line_num' => '18',
                'message' => '415 (384 direct, 31 indirect) bytes in 1 blocks are definitely lost in loss record 6 of 10',
                'trace' => nil
              }
            }
          ]
        ]
      end
    end
  end
end
