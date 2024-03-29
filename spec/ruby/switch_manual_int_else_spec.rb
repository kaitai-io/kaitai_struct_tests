# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'SwitchManualIntElse' do
  it 'parses test properly' do
    require 'switch_manual_int_else'
    r = SwitchManualIntElse.from_file('src/switch_opcodes2.bin')

    expect(r.opcodes.length).to eq 4
    expect(r.opcodes[0].code).to eq 83
    expect(r.opcodes[0].body.value).to eq "foo"
    expect(r.opcodes[1].code).to eq 88
    expect(r.opcodes[1].body.filler).to eq 66
    expect(r.opcodes[2].code).to eq 89
    expect(r.opcodes[2].body.filler).to eq 51966
    expect(r.opcodes[3].code).to eq 73
    expect(r.opcodes[3].body.value).to eq 7
  end
end
