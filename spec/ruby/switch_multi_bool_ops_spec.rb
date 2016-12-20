require 'switch_multi_bool_ops'

RSpec.describe SwitchMultiBoolOps do
  it 'parses test properly' do
    r = SwitchMultiBoolOps.from_file('src/switch_integers.bin')

    expect(r.opcodes.size).to eq 4

    expect(r.opcodes[0].code).to eq 1
    expect(r.opcodes[0].body).to eq 7

    expect(r.opcodes[1].code).to eq 2
    expect(r.opcodes[1].body).to eq 0x4040

    expect(r.opcodes[2].code).to eq 4
    expect(r.opcodes[2].body).to eq 4919

    expect(r.opcodes[3].code).to eq 8
    expect(r.opcodes[3].body).to eq 4919
  end
end
