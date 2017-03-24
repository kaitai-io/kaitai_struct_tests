require 'imports_circular_a'
require 'imports_circular_b'

RSpec.describe ImportsCircularA do
  it 'parses test properly' do
    r = ImportsCircularA.from_file('src/fixed_struct.bin')

    expect(r.code).to eq 0x50
    expect(r.two.initial).to eq 0x41
    expect(r.two.back_ref.code).to eq 0x43
    expect(r.two.back_ref.two.initial).to eq 0x4b
    expect(r.two.back_ref.two.back_ref).to eq nil
  end
end
