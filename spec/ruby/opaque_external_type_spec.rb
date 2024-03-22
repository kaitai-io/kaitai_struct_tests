RSpec.describe 'OpaqueExternalType' do
  it 'parses test properly' do
    require 'opaque_external_type'
    require 'hello_world' # the opaque type
    r = OpaqueExternalType.from_file('src/term_strz.bin')

    expect(r.hw.one).to eq 102
  end
end
