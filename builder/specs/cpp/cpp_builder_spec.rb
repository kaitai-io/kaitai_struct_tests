require_relative '../../cpp_builder'

RSpec.describe CppBuilder do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  context '1' do
    before :context do
      Dir.chdir("#{@spec_dir}/1")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
    end

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        expect(@builder.list_mandatory_files.to_a.sort).to eq []
      end
    end

    describe '#list_mandatory_files' do
      it 'shows no mandatory files' do
        expect(@builder.list_mandatory_files.to_a.sort).to eq []
      end
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for make' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').to_a.sort).to eq ['/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp']
      end

      it 'parses failed build information for ld' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-2.log').to_a.sort).to eq ['/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp']
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          @builder.file_to_test("/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp")
        ).to eq [:spec, 'IoLocalVar']
      end

      it 'parses format filename' do
        expect(
          @builder.file_to_test("/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp")
        ).to eq [:format, 'IoLocalVar']
      end
    end
  end

  context 'msbuild_1' do
    before :context do
      Dir.chdir("#{@spec_dir}/msbuild_1")
      @builder = CppBuilder.new('compiled/cpp_stl_98', 'spec/cpp_stl_98', 'test_out/cpp_stl_98')
      @builder.mode = :msbuild_windows
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for msbuild' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-1.log').to_a.sort).to match_array [
          "c:\\projects\\ci-targets\\tests\\compiled\\cpp_stl_98\\enum_to_i_class_border_2.cpp",
          "c:\\projects\\ci-targets\\tests\\compiled\\cpp_stl_98\\enum_to_i_class_border_2.h",
          "c:\\projects\\ci-targets\\tests\\compiled\\cpp_stl_98\\io_local_var.cpp",
        ]
      end
    end
  end

  context 'clang_osx_h' do
    before :context do
      Dir.chdir("#{@spec_dir}/clang_osx_h")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').to_a.sort).to match_array [
          '/Users/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/enum_to_i_class_border_2.cpp',
          '/Users/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/io_local_var.cpp',
        ]
      end
    end
  end

  context 'gcc_h' do
    before :context do
      Dir.chdir("#{@spec_dir}/gcc_h")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').to_a.sort).to match_array [
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/enum_to_i_class_border_2.cpp',
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/io_local_var.cpp',
        ]
      end
    end
  end
end
