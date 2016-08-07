require 'position_to_end'

RSpec.describe PositionToEnd do
  it 'parses test properly' do
    r = PositionToEnd.from_file('src/position_to_end.bin')

    expect(r.index.foo).to eq 0x42
    expect(r.index.bar).to eq 0x1234
  end
end
