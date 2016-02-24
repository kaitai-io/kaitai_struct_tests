require 'position_abs'

RSpec.describe PositionAbs do
  it 'parses test properly' do
    r = PositionAbs.from_file('src/position_abs.bin')

    expect(r.index_offset).to eq 0x20
    expect(r.index.entry).to eq "foo"
  end
end
