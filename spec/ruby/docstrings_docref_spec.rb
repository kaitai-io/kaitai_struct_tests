require 'docstrings_docref'

RSpec.describe DocstringsDocref do
  it 'parses test properly' do
    r = DocstringsDocref.from_file('src/fixed_struct.bin')
  end
end
