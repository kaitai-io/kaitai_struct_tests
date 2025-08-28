RSpec.describe 'DebugArrayUserCurrentExcluded' do
  it 'parses test properly' do
    require 'debug_array_user_current_excluded'
    r = DebugArrayUserCurrentExcluded.from_file('src/term_strz.bin')

    # --debug implies --no-auto-read
    r._read

    expect(r.array_of_cats[0].meow).to eq [102, 111, 111].pack('C*')
    expect(r.array_of_cats[1].meow).to eq [124, 98].pack('C*')
    expect(r.array_of_cats[2].meow).to eq [97].pack('C*')
  end
end
