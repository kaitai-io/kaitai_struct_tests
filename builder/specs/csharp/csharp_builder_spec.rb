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
        expect(r.parse_failed_build('test_out/csharp/build-1.log').to_a.sort).to eq ['/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp']
      end
    end
  end
end
