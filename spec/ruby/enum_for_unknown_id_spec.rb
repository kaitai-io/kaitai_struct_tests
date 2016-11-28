require 'enum_for_unknown_id'

RSpec.describe EnumForUnknownId do
  it 'parses test properly' do
    r = EnumForUnknownId.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 80
  end
end
