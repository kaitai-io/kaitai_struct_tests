require 'cast_to_top'

RSpec.describe CastToTop do
  it 'parses test properly' do
    r = CastToTop.from_file('src/fixed_struct.bin')

    expect(r.code).to eq 0x50
    expect(r.header.code).to eq 0x41
    expect(r.header_casted.code).to eq 0x41
  end
end
