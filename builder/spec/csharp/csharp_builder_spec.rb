require_relative '../../csharp_builder'
require 'pathname'

$spec_dir = File.dirname(__FILE__)

RSpec.describe CSharpBuilder do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  context '1' do
    before :context do
      Dir.chdir("#{@spec_dir}/1")
      @builder = CSharpBuilder.new
    end

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        base_dir = Pathname.pwd
        expect(@builder.list_mandatory_files.map { |x| Pathname.new(x).relative_path_from(base_dir).to_s }).to match_array [
          'spec/csharp/kaitai_struct_csharp_tests/CommonSpec.cs',
          'spec/csharp/kaitai_struct_csharp_tests/Properties/AssemblyInfo.cs',
        ]
      end
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/csharp/build-1.log').to_a.sort).to eq(
          ["\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecProcessCustom.cs"]
        )
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          @builder.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecProcessCustom.cs")
        ).to eq [:spec, 'ProcessCustom']
        expect(
          @builder.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecFooBar.cs")
        ).to eq [:spec, 'FooBar']
      end

      it 'parses format filename' do
        expect(
          @builder.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\compiled\\csharp\\NestedTypes.cs")
        ).to eq [:format, 'NestedTypes']
      end
    end
  end

  context '2' do
    before :context do
      Dir.chdir("#{@spec_dir}/2")
      @builder = CSharpBuilder.new
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/csharp/build-3.log').to_a.sort).to eq([
          [:bare, "SpecExprBytesOps.cs"],
        ])
      end
    end

    describe '#adjust_files_for_failed_build' do
      it 'adjusts build-3 log properly' do
        mand_files = Set.new(@builder.list_mandatory_files)
        disp_files = Set.new(@builder.list_disposable_files)
        bad_files = @builder.adjust_files_for_failed_build('test_out/csharp/build-3.log', mand_files, disp_files)
        expect(bad_files.to_a.sort).to eq([
          [:bare, "SpecExprBytesOps.cs"],
        ])
      end
    end

    describe '#file_to_test' do
      it 'parses bare file name' do
        expect(@builder.file_to_test([:bare, "SpecExprBytesOps.cs"])).to eq([:spec, "ExprBytesOps"])
      end
    end

    describe '#convert_slashes' do
      it 'parses bare file name' do
        expect(@builder.convert_slashes([
          "a/b/c/FooBar.cs",
          [:bare, "SpecExprBytesOps.cs"],
        ])).to eq([
          [:bare, "SpecExprBytesOps.cs"],
          "a\\b\\c\\FooBar.cs",
        ])
      end
    end
  end

  context '3' do
    before :context do
      Dir.chdir("#{@spec_dir}/3")
      @builder = CSharpBuilder.new
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        r = @builder.parse_failed_build('test_out/csharp/build-2.log')
        r = @builder.sort_list_with_bare(r.to_a)
        expect(r).to eq([
          "C:\\projects\\ci-targets\\tests\\compiled\\csharp\\EnumToIClassBorder2.cs",
          "C:\\projects\\ci-targets\\tests\\compiled\\csharp\\ExprBytesOps.cs",
          "C:\\projects\\ci-targets\\tests\\compiled\\csharp\\YamlInts.cs",
          [:bare, "SpecEofExceptionBytes.cs"],
          [:bare, "SpecEofExceptionU4.cs"],
          [:bare, "SpecEosExceptionBytes.cs"],
          [:bare, "SpecEosExceptionU4.cs"],
          [:bare, "SpecExprCalcArrayOps.cs"],
        ])
      end
    end
  end
end
