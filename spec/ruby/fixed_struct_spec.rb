require 'fixed_struct'

RSpec.describe Fixed_structFormat do
  it 'parses test properly' do
    r = Fixed_structFormat.new('src/fixed_struct.bin')

    expect(r.header.uint8).to eq 255
    expect(r.header.uint16).to eq 65535
    expect(r.header.uint32).to eq 4294967295
    expect(r.header.uint64).to eq 18446744073709551615

    expect(r.header.sint8).to eq -1
    expect(r.header.sint16).to eq -1
    expect(r.header.sint32).to eq -1
    expect(r.header.sint64).to eq -1

    expect(r.header.uint16le).to eq 66
    expect(r.header.uint32le).to eq 66
    expect(r.header.uint64le).to eq 66

    expect(r.header.sint16le).to eq -66
    expect(r.header.sint32le).to eq -66
    expect(r.header.sint64le).to eq -66

    expect(r.header.uint16be).to eq 66
    expect(r.header.uint32be).to eq 66
    expect(r.header.uint64be).to eq 66

    expect(r.header.sint16be).to eq -66
    expect(r.header.sint32be).to eq -66
    expect(r.header.sint64be).to eq -66
  end
end
