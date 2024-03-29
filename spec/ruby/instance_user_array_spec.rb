# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'InstanceUserArray' do
  it 'parses test properly' do
    require 'instance_user_array'
    r = InstanceUserArray.from_file('src/instance_std_array.bin')

    expect(r.ofs).to eq 16
    expect(r.qty_entries).to eq 3
    expect(r.entry_size).to eq 4
    expect(r.user_entries.length).to eq 3
    expect(r.user_entries[0].word1).to eq 4369
    expect(r.user_entries[0].word2).to eq 4369
    expect(r.user_entries[1].word1).to eq 8738
    expect(r.user_entries[1].word2).to eq 8738
    expect(r.user_entries[2].word1).to eq 13107
    expect(r.user_entries[2].word2).to eq 13107
  end
end
