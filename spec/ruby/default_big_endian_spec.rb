# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'DefaultBigEndian' do
  it 'parses test properly' do
    require 'default_big_endian'
    r = DefaultBigEndian.from_file('src/enum_0.bin')

    expect(r.one).to eq 117440512
  end
end
