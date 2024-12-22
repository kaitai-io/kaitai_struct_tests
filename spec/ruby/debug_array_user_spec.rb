RSpec.describe 'DebugArrayUser' do
  it 'parses test properly' do
    require 'debug_array_user'
    r = DebugArrayUser.from_file('src/fixed_struct.bin')

    # --debug implies --no-auto-read
    r._read

    expect(r.one_cat.meow).to eq 80
    expect(r.array_of_cats.length).to eq 3
    expect(r.array_of_cats[0].meow).to eq 65
    expect(r.array_of_cats[1].meow).to eq 67
    expect(r.array_of_cats[2].meow).to eq 75
  end
end
