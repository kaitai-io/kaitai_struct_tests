RSpec.describe 'OptionalId' do
  it 'parses test properly' do
    require 'optional_id'
    r = OptionalId.from_file('src/fixed_struct.bin')

    expect(r._unnamed0).to eq 80
    expect(r._unnamed1).to eq 65
    expect(r._unnamed2).to eq [67, 75, 45, 49, 255].pack('C*')
  end
end
