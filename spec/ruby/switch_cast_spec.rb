RSpec.describe 'SwitchCast' do
  it 'parses test properly' do
    require 'switch_cast'
    r = SwitchCast.from_file('src/switch_opcodes.bin')

    expect(r.first_obj.value).to eq 'foobar'
    expect(r.second_val).to eq 0x42
    # unable to test "err_cast" here
  end
end
