# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'RepeatNStrz' do
  it 'parses test properly' do
    require 'repeat_n_strz'
    r = RepeatNStrz.from_file('src/repeat_n_strz.bin')

    expect(r.qty).to eq 2
    expect(r.lines).to eq ["foo", "bar"]
  end
end
