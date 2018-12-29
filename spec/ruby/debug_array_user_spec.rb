require 'debug_array_user'

RSpec.describe DebugArrayUser do
  it 'parses test properly' do
    r = DebugArrayUser.from_file('src/fixed_struct.bin')
    r._read

    expect(r.one_cat.meow).to eq 0x50
    expect(r.array_of_cats[0].meow).to eq 0x41
    expect(r.array_of_cats[1].meow).to eq 0x43
    expect(r.array_of_cats[2].meow).to eq 0x4b
  end
end
