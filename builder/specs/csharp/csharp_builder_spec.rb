require_relative '../../csharp_builder'

$spec_dir = File.dirname(__FILE__)

RSpec.describe CSharpBuilder do
  context '1' do
    Dir.chdir("#{$spec_dir}/1")
    r = CSharpBuilder.new

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        expect(r.list_mandatory_files.to_a.sort).to eq ["CommonSpec.cs", "Properties\\AssemblyInfo.cs"]
      end
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(r.parse_failed_build('test_out/csharp/build-1.log').to_a.sort).to eq(
          ["\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecProcessCustom.cs"]
        )
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          r.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecProcessCustom.cs")
        ).to eq [:spec, 'ProcessCustom']
        expect(
          r.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\spec\\csharp\\kaitai_struct_csharp_tests\\tests\\SpecFooBar.cs")
        ).to eq [:spec, 'FooBar']
      end

      it 'parses format filename' do
        expect(
          r.file_to_test("\\home\\greycat\\git\\kaitai_struct\\tests\\compiled\\csharp\\NestedTypes.cs")
        ).to eq [:format, 'NestedTypes']
      end
    end
  end
end
