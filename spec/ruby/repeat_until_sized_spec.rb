require 'repeat_until_sized'

RSpec.describe RepeatUntilSized do
  it 'parses test properly' do
    r = RepeatUntilSized.from_file('src/repeat_until_process.bin')

    expect(r.records.length).to eq 3

    expect(r.records[0].marker).to eq 0xe8
    expect(r.records[0].body).to eq 0xaaaaaaba

    expect(r.records[1].marker).to eq 0xfa
    expect(r.records[1].body).to eq 0xaaaab89e

    expect(r.records[2].marker).to eq 0xaa
    expect(r.records[2].body).to eq 0x55555555
  end
end
