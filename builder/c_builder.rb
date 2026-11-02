require 'fileutils'

require_relative 'cpp_builder'

class CBuilder < CppBuilder
  attr_accessor :mode

  def initialize(src_dir, cpp_spec_dir, cpp_test_out_dir)
    super(src_dir, cpp_spec_dir, cpp_test_out_dir)
  end

  def list_disposable_files
    list = Dir.glob("#{@cpp_spec_dir}/**/*.c") + Dir.glob("#{@src_dir}/*.c") +  Dir.glob("#{@cpp_spec_dir}/**/*.cpp") + Dir.glob("#{@src_dir}/*.cpp")
    list.map { |x|
      r = File.absolute_path(x)

      # On Windows, filesystem is case insensitive, but our Set
      # implementation is not. So we proactively normalize everything we
      # can to lower case.
      if @mode == :msbuild_windows
        r.downcase!
      end

      r
    }
  end
end
