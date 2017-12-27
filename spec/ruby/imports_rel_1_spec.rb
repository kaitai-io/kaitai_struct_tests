require 'imports_rel_1'
require 'imported_1'
require 'imported_2'

RSpec.describe ImportsRel1 do
  it 'parses test properly' do
    r = ImportsRel1.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 0x50
    expect(r.two.one).to eq 0x41
    expect(r.two.two.one).to eq 0x43
  end
end
