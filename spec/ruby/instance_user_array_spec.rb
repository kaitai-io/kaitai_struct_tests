# coding: utf-8
require 'instance_user_array'

RSpec.describe InstanceUserArray do
  it 'parses test properly' do
    r = InstanceUserArray.from_file('src/instance_std_array.bin')

    expect(r.ofs).to eq 0x10
    expect(r.qty_entries).to eq 3
    expect(r.entry_size).to eq 4

    expect(r.entries[0].word1).to eq 0x1111
    expect(r.entries[0].word2).to eq 0x1111
    expect(r.entries[1].word1).to eq 0x2222
    expect(r.entries[1].word2).to eq 0x2222
    expect(r.entries[2].word1).to eq 0x3333
    expect(r.entries[2].word2).to eq 0x3333
  end
end
