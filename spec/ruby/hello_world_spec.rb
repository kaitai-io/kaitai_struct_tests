require 'hello_world'

RSpec.describe HelloWorld do
  it 'parses test properly' do
    r = HelloWorld.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 0x50
  end
end
