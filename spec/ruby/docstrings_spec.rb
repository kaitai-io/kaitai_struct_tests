require 'docstrings'

RSpec.describe Docstrings do
  it 'parses test properly' do
    r = Docstrings.from_file('src/fixed_struct.bin')
  end
end
