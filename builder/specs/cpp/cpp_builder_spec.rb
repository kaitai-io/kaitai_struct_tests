require_relative '../../cpp_builder'

$spec_dir = File.dirname(__FILE__)

RSpec.describe CppBuilder do
  context '1' do
    Dir.chdir("#{$spec_dir}/1")
    r = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        expect(r.list_mandatory_files.to_a.sort).to eq []
      end
    end

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        expect(r.list_mandatory_files.to_a.sort).to eq []
      end
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for make' do
        p Dir.pwd
        expect(r.parse_failed_build('test_out/cpp_stl_11/build-1.log').to_a.sort).to eq ['/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp']
      end

      it 'parses failed build information for ld' do
        expect(r.parse_failed_build('test_out/cpp_stl_11/build-2.log').to_a.sort).to eq ['/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp']
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          r.file_to_test("/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp")
        ).to eq [:spec, 'IoLocalVar']
      end

      it 'parses format filename' do
        expect(
          r.file_to_test("/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp")
        ).to eq [:format, 'IoLocalVar']
      end
    end
  end
end
