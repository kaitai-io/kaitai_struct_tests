# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'IfValues' do
  it 'parses test properly' do
    require 'if_values'
    r = IfValues.from_file('src/fixed_struct.bin')

    expect(r.codes[0].opcode).to eq 80
    expect(r.codes[0].half_opcode).to eq 40
    expect(r.codes[1].opcode).to eq 65
    expect(r.codes[1].half_opcode).to be_nil
    expect(r.codes[2].opcode).to eq 67
    expect(r.codes[2].half_opcode).to be_nil
  end
end
