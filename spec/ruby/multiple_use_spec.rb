require 'multiple_use'

RSpec.describe MultipleUse do
  it 'parses test properly' do
    r = MultipleUse.from_file('src/position_abs.bin')

    expect(r.t1.first_use.value).to eq 0x20
    expect(r.t2.second_use.value).to eq 0x20
  end
end
