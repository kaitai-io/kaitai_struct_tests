require 'switch_manual_str'

RSpec.describe SwitchManualStr do
  it 'parses test properly' do
    r = SwitchManualStr.from_file('src/switch_opcodes.bin')

    expect(r.opcodes.size).to eq 4

    expect(r.opcodes[0].code).to eq 'S'
    expect(r.opcodes[0].body.value).to eq 'foobar'

    expect(r.opcodes[1].code).to eq 'I'
    expect(r.opcodes[1].body.value).to eq 0x42

    expect(r.opcodes[2].code).to eq 'I'
    expect(r.opcodes[2].body.value).to eq 0x37

    expect(r.opcodes[3].code).to eq 'S'
    expect(r.opcodes[3].body.value).to eq ''
  end
end
