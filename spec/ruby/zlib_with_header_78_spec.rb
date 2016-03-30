require 'zlib_with_header_78'

RSpec.describe ZlibWithHeader78 do
  it 'parses test properly' do
    r = ZlibWithHeader78.from_file('src/zlib_with_header_78.bin')

    expect(r.data).to eq 'a quick brown fox jumps over'
  end
end
