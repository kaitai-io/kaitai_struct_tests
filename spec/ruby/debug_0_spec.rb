require 'debug_0'

RSpec.describe Debug0 do
  it 'parses test properly' do
    r = Debug0.from_file('src/fixed_struct.bin')
    r._read

    expect(r.one).to eq 0x50
    expect(r.array_of_ints).to eq [0x41, 0x43, 0x4b]

    expect(Debug0::SEQ_FIELDS).to eq ['one', 'array_of_ints', '_unnamed2']
    expect(r._debug['one']).to eq({
      start: 0,
      end: 1,
    })
    expect(r._debug['array_of_ints']).to eq({
      start: 1,
      end: 4,
      arr: [
        {start: 1, end: 2},
        {start: 2, end: 3},
        {start: 3, end: 4},
      ]
    })
  end
end
