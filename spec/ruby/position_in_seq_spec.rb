require 'position_in_seq'

RSpec.describe PositionInSeq do
  it 'parses test properly' do
    r = PositionInSeq.from_file('src/position_in_seq.bin')

    expect(r.numbers).to eq [1, 2, 3]
  end
end
