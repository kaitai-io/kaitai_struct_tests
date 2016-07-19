class BuildFailParser
  def initialize(dir)
    @dir = dir
  end

  def each_test
    File.open("#{@dir}/build.fails", 'r') { |f|
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
    }
  end
end
