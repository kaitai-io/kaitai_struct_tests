require 'cast_to_imported'

RSpec.describe CastToImported do
  it 'parses test properly' do
    r = CastToImported.from_file('src/fixed_struct.bin')

    expect(r.one.one).to eq 0x50
    expect(r.one_casted.one).to eq 0x50
  end
end
