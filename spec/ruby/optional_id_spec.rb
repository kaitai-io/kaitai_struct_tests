require 'optional_id'

RSpec.describe OptionalId do
  it 'parses test properly' do
    r = OptionalId.from_file('src/fixed_struct.bin')

    expect(r._unnamed0).to eq 0x50
    expect(r._unnamed1).to eq 0x41
    expect(r._unnamed2).to eq [0x43, 0x4b, 0x2d, 0x31, 0xff].pack('C*')
  end
end
