# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'RepeatUntilS4' do
  it 'parses test properly' do
    require 'repeat_until_s4'
    r = RepeatUntilS4.from_file('src/repeat_until_s4.bin')

    expect(r.entries).to eq [66, 4919, -251658241, -1]
    expect(r.afterall).to eq "foobar"
  end
end
