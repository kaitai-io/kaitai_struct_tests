require 'switch_repeat_expr'

RSpec.describe SwitchRepeatExpr do
  it 'parses test properly' do
    r = SwitchRepeatExpr.from_file('src/switch_tlv.bin')

    expect(r.code).to eq 0x11
    expect(r.size).to eq 9
    expect(r.body[0].first).to eq "Stuff\0Me\0"
  end
end
