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
end
