require 'process_coerce_usertype1'

RSpec.describe ProcessCoerceUsertype1 do
  it 'parses test properly' do
    r = ProcessCoerceUsertype1.from_file('src/process_coerce_bytes.bin')

    expect(r.records[0].flag).to eq 0
    expect(r.records[0].buf.value).to eq 0x41414141
    expect(r.records[1].flag).to eq 1
    expect(r.records[1].buf.value).to eq 0x42424242
  end
end
