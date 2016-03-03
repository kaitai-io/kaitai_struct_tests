# coding: utf-8
require 'instance_std_array'

RSpec.describe InstanceStdArray do
  it 'parses test properly' do
    r = InstanceStdArray.from_file('src/instance_std_array.bin')

    expect(r.ofs).to eq 0x10
    expect(r.qty_entries).to eq 3
    expect(r.entry_size).to eq 4

    expect(r.entries).to eq [
      "\x11\x11\x11\x11",
      "\x22\x22\x22\x22",
      "\x33\x33\x33\x33",
    ]
  end
end
