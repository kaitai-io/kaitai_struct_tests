require_relative '../../cpp_builder'
require 'rspec' # normally not needed, but RubyMine doesn't autocomplete RSpec methods without it

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
        expect(@builder.list_mandatory_files).to eq []
      end
    end

    describe '#list_disposable_files' do
      it 'shows disposable files' do
        expect(@builder.list_disposable_files.map { |x| File.basename(x) }).to match_array [
          'bcd_user_type_be.cpp',
          'bcd_user_type_le.cpp',
          'io_local_var.cpp',
        ]
      end
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for make' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').uniq).to eq ['/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp']
      end

      it 'parses failed build information for ld' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-2.log')).to eq ['/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp']
      end
    end

    describe '#adjust_files_for_failed_build' do
      it 'properly disposes files' do
        mand_files = Set.new
        disp_files = Set.new([
          '/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp',
          '/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/foo.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_foo.cpp',
        ])

        r = @builder.adjust_files_for_failed_build('test_out/cpp_stl_11/build-1.log', mand_files, disp_files)

        expect(r).to be_truthy
        expect(disp_files).to match_array [
          # '/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp',
          '/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/foo.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_foo.cpp',
        ]
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          @builder.file_to_test('/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_11/test_io_local_var.cpp')
        ).to eq [:spec, 'IoLocalVar']
      end

      it 'parses format filename' do
        expect(
          @builder.file_to_test('/home/greycat/git/kaitai_struct/tests/compiled/cpp_stl_11/io_local_var.cpp')
        ).to eq [:format, 'IoLocalVar']
      end

      it 'parses bare spec filename' do
        expect(
          @builder.file_to_test([:bare, 'test_io_local_var.cpp'])
        ).to eq [:spec, 'IoLocalVar']
      end
    end
  end

  context '2' do
    before :context do
      Dir.chdir("#{@spec_dir}/2")
      @builder = CppBuilder.new('compiled/cpp_stl_98', 'spec/cpp_stl_98', 'test_out/cpp_stl_98')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for ld' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-2.log')).to eq [
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_imports_abs.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_imports_abs.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_cast_nested.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_cast_nested.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_cast_nested.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_cast_nested.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_cast_nested.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_repeat_until_sized.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_io_local_var.cpp',
          '/home/greycat/git/kaitai_struct/tests/spec/cpp_stl_98/test_repeat_n_struct.cpp',
        ]
      end
    end
  end

  context 'msbuild_1' do
    before :context do
      Dir.chdir("#{@spec_dir}/msbuild_1")
      @builder = CppBuilder.new('compiled/cpp_stl_98', 'spec/cpp_stl_98', 'test_out/cpp_stl_98')
      # Comment out the following line to enable logging for this test
      def @builder.log(msg); end
      @builder.mode = :msbuild_windows
    end

    describe '#parse_failed_build' do
      it 'parses failed build information for msbuild' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-1.log').uniq).to eq [
          [:bare, 'enum_to_i_class_border_2.cpp'],
          'c:/projects/ci-targets/tests/compiled/cpp_stl_98/enum_to_i_class_border_2.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_98/io_local_var.cpp',
        ]
      end

      it 'parses failed link information for msbuild' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-2.log')).to eq [
          [:bare, 'test_io_local_var.cpp'],
          [:bare, 'enum_to_i_class_border_1.cpp'],
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
      it 'parses failed build information (compiler)' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').uniq).to eq [
          '/Users/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/enum_to_i_class_border_2.cpp',
          '/Users/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/io_local_var.cpp',
        ]
      end

      it 'parses failed build information (linker)' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-2.log')).to eq [
          [:bare, 'test_io_local_var.cpp'],
          [:bare, 'enum_to_i_class_border_1.cpp'],
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
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').uniq).to eq [
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/io_local_var.cpp',
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_11/enum_to_i_class_border_2.cpp',
        ]
      end
    end
  end

  context 'gcc11' do
    before :context do
      Dir.chdir("#{@spec_dir}/gcc11")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        # FIXME: this monkey patching is ugly and fragile, but I don't know how to do this better
        allow(File).to receive(:absolute_path) do |arg|
          File.join('/tests', arg)
        end

        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-1.log').uniq).to eq [
          '/tests/spec/cpp_stl_11/test_enum_import.cpp',
          '/tests/spec/cpp_stl_11/test_expr_bytes_non_literal.cpp',
          '/tests/spec/cpp_stl_11/test_process_coerce_switch.cpp',
          '/tests/compiled/cpp_stl_11/debug_switch_user.cpp',
          '/tests/compiled/cpp_stl_11/enum_import.cpp',
          '/tests/compiled/cpp_stl_11/enum_to_i.cpp',
          '/tests/compiled/cpp_stl_11/enum_to_i_class_border_1.cpp',
          '/tests/compiled/cpp_stl_11/enum_to_i_class_border_2.cpp',
          '/tests/compiled/cpp_stl_11/enum_to_i_invalid.cpp',
          '/tests/compiled/cpp_stl_11/expr_ops_parens.cpp',
          '/tests/compiled/cpp_stl_11/nav_parent_switch_cast.cpp',
          '/tests/compiled/cpp_stl_11/params_pass_array_usertype.cpp',
        ]
      end
    end
  end

  # The log comes from https://ci.appveyor.com/project/kaitai-io/ci-targets/builds/31957431/job/e5q10o8urou743d9
  context 'msbuild_h' do
    before :context do
      Dir.chdir("#{@spec_dir}/msbuild_h")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
      # Comment out the following line to enable logging for this test
      def @builder.log(msg); end
      @builder.mode = :msbuild_windows
    end

    describe '#parse_failed_build' do
      it 'parses failed build info' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-2.log')).to eq [
          [:bare, 'test_imports_abs.cpp'],
          [:bare, 'test_imports_abs_abs.cpp'],
          [:bare, 'test_imports_abs_rel.cpp'],
          [:bare, 'test_imports_rel_1.cpp'],
          [:bare, 'imported_and_abs.cpp'],
          [:bare, 'imports_abs.cpp'],
          [:bare, 'imports_abs_abs.cpp'],
          [:bare, 'imports_abs_rel.cpp'],
          [:bare, 'imports_rel_1.cpp'],
          [:bare, 'switch_else_only.cpp'],
          [:bare, 'switch_else_only.cpp'],
          [:bare, 'switch_else_only.cpp'],
        ]
      end
    end

    describe '#adjust_files_for_failed_build' do
      it 'properly disposes files' do
        mand_files = Set.new([
          'c:/projects/ci-targets/runtime/cpp_stl/kaitai/kaitaistream.cpp',
        ])
        disp_files = Set.new([
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_combine_str.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_debug_array_user.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_enum_int_range_s.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_expr_bytes_ops.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_if_values.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs_abs.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs_rel.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_circular_a.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_rel_1.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_index_sizes.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_meta_xref.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_opaque_with_param.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_process_coerce_usertype1.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_process_repeat_bytes.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_str_eos.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_switch_integers.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_switch_multi_bool_ops.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_valid_short.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/debug_enum_name.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/default_big_endian.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/enum_int_range_u.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/eof_exception_bytes.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_bytes_ops.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_enum.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_sizeof_type_0.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_1.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_and_abs.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_and_rel.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs_abs.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs_rel.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_circular_a.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_rel_1.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/index_sizes.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/integers.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/nav_parent_vs_value_inst.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/nested_types2.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/params_pass_struct.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/position_to_end.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/process_repeat_usertype.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/repeat_n_strz_double.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/repeat_until_complex.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_else_only.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_integers.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int_size.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int_size_eos.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_multi_bool_ops.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/ts_packet_header.cpp',
        ])

        r = @builder.adjust_files_for_failed_build('test_out/cpp_stl_11/build-2.log', mand_files, disp_files)

        expect(r).to be_truthy
        expect(disp_files).to match_array [
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_combine_str.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_debug_array_user.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_enum_int_range_s.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_expr_bytes_ops.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_if_values.cpp',
          # "c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs.cpp",
          # "c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs_abs.cpp",
          # "c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_abs_rel.cpp",
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_circular_a.cpp',
          # "c:/projects/ci-targets/tests/spec/cpp_stl_11/test_imports_rel_1.cpp",
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_index_sizes.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_meta_xref.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_opaque_with_param.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_process_coerce_usertype1.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_process_repeat_bytes.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_str_eos.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_switch_integers.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_switch_multi_bool_ops.cpp',
          'c:/projects/ci-targets/tests/spec/cpp_stl_11/test_valid_short.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/debug_enum_name.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/default_big_endian.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/enum_int_range_u.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/eof_exception_bytes.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_bytes_ops.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_enum.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/expr_sizeof_type_0.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_1.cpp',
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_and_abs.cpp",
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imported_and_rel.cpp',
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs.cpp",
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs_abs.cpp",
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_abs_rel.cpp",
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_circular_a.cpp',
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/imports_rel_1.cpp",
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/index_sizes.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/integers.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/nav_parent_vs_value_inst.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/nested_types2.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/params_pass_struct.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/position_to_end.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/process_repeat_usertype.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/repeat_n_strz_double.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/repeat_until_complex.cpp',
          # "c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_else_only.cpp",
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_integers.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int_size.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_manual_int_size_eos.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/switch_multi_bool_ops.cpp',
          'c:/projects/ci-targets/tests/compiled/cpp_stl_11/ts_packet_header.cpp',
        ]
      end
    end
  end

  context 'clang_linux' do
    before :context do
      Dir.chdir("#{@spec_dir}/clang_linux")
      @builder = CppBuilder.new('compiled/cpp_stl_98', 'spec/cpp_stl_98', 'test_out/cpp_stl_98')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-1.log').uniq).to eq [
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_98/io_local_var.cpp',
          '/home/travis/build/kaitai-io/ci_targets/tests/compiled/cpp_stl_98/enum_to_i_class_border_2.cpp',
        ]
      end
    end
  end

  context 'gcc_enoent' do
    before :context do
      Dir.chdir("#{@spec_dir}/gcc_enoent")
      @builder = CppBuilder.new('compiled/cpp_stl_11', 'spec/cpp_stl_11', 'test_out/cpp_stl_11')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        # FIXME: this monkey patching is ugly and fragile, but I don't know how to do this better
        allow(File).to receive(:absolute_path) do |arg|
          File.join('/home/travis/build/kaitai-io/ci_targets/tests', arg)
        end

        expect(@builder.parse_failed_build('test_out/cpp_stl_11/build-2.log')).to eq [
          '/home/travis/build/kaitai-io/ci_targets/tests/spec/cpp_stl_11/test_expr_calc_array_ops.cpp',
        ]
      end
    end
  end

  context 'msbuild_enoent' do
    before :context do
      Dir.chdir("#{@spec_dir}/msbuild_enoent")
      @builder = CppBuilder.new('compiled/cpp_stl_98', 'spec/cpp_stl_98', 'test_out/cpp_stl_98')
      # Comment out the following line to enable logging for this test
      def @builder.log(msg); end
      @builder.mode = :msbuild_windows
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/cpp_stl_98/build-2.log')).to eq [
          'c:/projects/ci-targets/tests/spec/cpp_stl_98/test_expr_calc_array_ops.cpp',
        ]
      end
    end
  end
end
