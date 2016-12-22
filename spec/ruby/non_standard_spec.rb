require 'non_standard'

RSpec.describe NonStandard do
  it 'parses test properly' do
    r = NonStandard.from_file('src/fixed_struct.bin')

    expect(r.foo).to eq 0x50
  end
end
