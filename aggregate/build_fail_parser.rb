class BuildFailParser
  def initialize(dir)
    @dir = dir
  end

  def each_test
    fn = "#{@dir}/build.fails"
    File.open(fn, 'r') { |f|
      f.each_line { |l|
        case l
        when /^Test(.*?)\.java$/
          name = $1
          yield TestResult.new(name, :spec_build_failed, 0, nil)
        when /^(.*?)\.java$/
          name = $1
          yield TestResult.new(name, :format_build_failed, 0, nil)
        end
      }
    } if FileTest::exists?(fn)
  end
end
