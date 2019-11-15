require 'imports_abs_rel'
require 'imported_and_rel'
require 'imported_root'

RSpec.describe ImportsAbsRel do
  it 'parses test properly' do
    r = ImportsAbsRel.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 0x50
    expect(r.two.one).to eq 0x41
    expect(r.two.two.one).to eq 0x43
  end
end
