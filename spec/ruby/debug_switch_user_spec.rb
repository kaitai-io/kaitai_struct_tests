# runs in debug mode, so the _read() needs to be called manually

RSpec.describe 'DebugSwitchUser' do
  it 'parses test properly' do
    require 'debug_switch_user'
    r = DebugSwitchUser.from_file('src/nav_parent_switch.bin')
    r._read

    expect(r.code).to eq 1
    expect(r.data.val).to eq -190
  end
end
