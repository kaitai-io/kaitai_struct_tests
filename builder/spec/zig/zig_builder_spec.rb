require_relative '../../zig_builder'

# From https://stackoverflow.com/a/64278755
RSpec.configure do |rspec|
  rspec.expect_with :rspec do |c|
    c.max_formatted_output_length = nil
  end
end

RSpec.describe ZigBuilder do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  # This scenario shows that the Zig compiler only reports the file where the
  # syntax error is found, without indicating where the file was imported from,
  # as is the case with type mismatch errors in the "referenced by:" trace - see
  # the test case "format_imports_with_type_mismatch". This forces us to
  # physically remove the format *.zig files with these errors (we do this by
  # renaming them to *.zig.DISABLED), which causes "unable to load 'format.zig':
  # FileNotFound" errors in the files importing such faulty formats in the next
  # build attempt.
  context 'format_imports_with_syntax_error' do
    before :context do
      Dir.chdir("#{@spec_dir}/format_imports_with_syntax_error")
      @builder = ZigBuilder.new
    end

    describe '#parse_failed_build' do
      it 'identifies the imported format with syntax error' do
        expect(@builder.parse_failed_build('test_out/zig/build-1.log')).to eq [
          File.absolute_path('spec/zig/formats/imported_with_syntax_error.zig'),
        ]
      end
    end
  end

  context 'format_imports_with_type_mismatch' do
    before :context do
      Dir.chdir("#{@spec_dir}/format_imports_with_type_mismatch")
      @builder = ZigBuilder.new
    end

    describe '#parse_failed_build' do
      it 'identifies the imported format with type mismatch + the test spec that references it' do
        expect(@builder.parse_failed_build('test_out/zig/build-1.log')).to eq [
          File.absolute_path('spec/zig/formats/imported_with_type_mismatch.zig'),
          File.absolute_path('spec/zig/formats/imported_with_type_mismatch.zig'),
          File.absolute_path('spec/zig/formats/format_imports_with_type_mismatch.zig'),
          File.absolute_path('spec/zig/formats/format_imports_with_type_mismatch.zig'),
          File.absolute_path('spec/zig/tests/format_imports_with_type_mismatch_test.zig'),
        ]
      end
    end
  end

  context 'spec_format_enoent' do
    before :context do
      Dir.chdir("#{@spec_dir}/spec_format_enoent")
      @builder = ZigBuilder.new
    end

    describe '#parse_failed_build' do
      it 'identifies the test spec that attempts to import a non-existent format' do
        expect(@builder.parse_failed_build('test_out/zig/build-1.log')).to eq [
          File.absolute_path('spec/zig/tests/spec_format_enoent_test.zig'),
        ]
      end
    end
  end

  context 'spec_type_mismatch_expect_equal' do
    before :context do
      Dir.chdir("#{@spec_dir}/spec_type_mismatch_expect_equal")
      @builder = ZigBuilder.new
    end

    describe '#parse_failed_build' do
      it 'identifies the test spec with type mismatch in testing.expectEqual() call' do
        expect(@builder.parse_failed_build('test_out/zig/build-1.log')).to eq [
          File.absolute_path('spec/zig/tests/spec_type_mismatch_expect_equal_test.zig'),
        ]
      end
    end
  end

  context 'spec_type_mismatch_format_param' do
    before :context do
      Dir.chdir("#{@spec_dir}/spec_type_mismatch_format_param")
      @builder = ZigBuilder.new
    end

    describe '#parse_failed_build' do
      it 'identifies the test spec (but not the format!) with type mismatch in format parameter' do
        expect(@builder.parse_failed_build('test_out/zig/build-1.log')).to eq [
          File.absolute_path('spec/zig/tests/spec_type_mismatch_format_param_test.zig'),
        ]
      end
    end
  end
end
