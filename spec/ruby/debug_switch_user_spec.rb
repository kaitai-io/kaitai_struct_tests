RSpec.describe 'DebugSwitchUser' do
  it 'parses test properly' do
    require 'debug_switch_user'
    r = DebugSwitchUser.from_file('src/nav_parent_switch.bin')

    # --debug implies --no-auto-read
    r._read

    expect(r.code).to eq 1
    expect(r.data.val).to eq -190
  end
end
