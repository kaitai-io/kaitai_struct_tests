require 'imports0'

RSpec.describe Imports0 do
  it 'parses test properly' do
    r = Imports0.from_file('src/fixed_struct.bin')

    expect(r.two).to eq 0x50
    expect(r.hw.one).to eq 0x41
    expect(r.hw_one).to eq 0x41
  end
end
