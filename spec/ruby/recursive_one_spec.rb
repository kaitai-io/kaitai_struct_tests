require 'recursive_one'

RSpec.describe RecursiveOne do
  it 'parses test properly' do
    r = RecursiveOne.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 0x50
    expect(r.next.one).to eq 0x41
    expect(r.next.next.one).to eq 0x43
    expect(r.next.next.next.finisher).to eq 0x2d4b
  end
end
