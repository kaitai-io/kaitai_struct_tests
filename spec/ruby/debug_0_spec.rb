RSpec.describe 'Debug0' do
  it 'parses test properly' do
    require 'debug_0'
    r = Debug0.from_file('src/fixed_struct.bin')
    r._read

    expect(r.one).to eq 80
    expect(r.array_of_ints).to eq [65, 67, 75]
    expect(r._unnamed2).to eq 45

    expect(Debug0::SEQ_FIELDS).to eq ['one', 'array_of_ints', '_unnamed2']
    expect(r._debug).to eq({
      'one' => {
        start: 0,
        end: 1,
      },
      'array_of_ints' => {
        start: 1,
        end: 4,
        arr: [
          {start: 1, end: 2},
          {start: 2, end: 3},
          {start: 3, end: 4},
        ],
      },
      '_unnamed2' => {
        start: 4,
        end: 5,
      },
    })
  end
end
