class CustomFxNoArgs
  def initialize()
  end

  def decode(data)
    "_" + data + "_"
  end
end

RSpec.describe 'ProcessCustomNoArgs' do
  it 'parses test properly' do
    require 'process_custom_no_args'
    r = ProcessCustomNoArgs.from_file('src/process_rotate.bin')

    expect(r.buf).to eq [95, 9, 172, 141, 141, 237, 95].pack('C*')
  end
end
