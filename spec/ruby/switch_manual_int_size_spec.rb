require 'switch_manual_int_size'

RSpec.describe SwitchManualIntSize do
  it 'parses test properly' do
    r = SwitchManualIntSize.from_file('src/switch_tlv.bin')

    expect(r.chunks.size).to eq 4

    expect(r.chunks[0].code).to eq 0x11
    meta = r.chunks[0].body
    expect(meta.title).to eq 'Stuff'
    expect(meta.author).to eq 'Me'

    expect(r.chunks[1].code).to eq 0x22
    expect(r.chunks[1].body.entries).to eq ['AAAA', 'BBBB', 'CCCC']

    expect(r.chunks[2].code).to eq 0x33
    expect(r.chunks[2].body).to eq [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80].pack('C*')

    expect(r.chunks[3].code).to eq 0xff
    expect(r.chunks[3].body).to eq ''
  end
end
