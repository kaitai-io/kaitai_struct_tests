class TestResult
  class Failure
    attr_reader :file_name
    attr_reader :line_num
    attr_reader :message
    attr_reader :trace

    def initialize(file_name, line_num, message, trace)
      @file_name = file_name
      @line_num = line_num
      @message = message
      @trace = trace
    end
  end

  attr_reader :name
  attr_reader :status
  attr_reader :elapsed
  attr_reader :failure

  def initialize(name, status, elapsed, failure = nil)
    @name = name
    @status = status
    @elapsed = elapsed
    @failure = failure
  end
end
