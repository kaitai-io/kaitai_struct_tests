class MyCustomFx
  def initialize(key, flag, some_bytes)
    @key = flag ? key : -key
  end

  def decode(data)
    data.bytes.map { |x| (x + @key) % 0x100 }.pack('C*')
  end
end

module Nested
  module Deeply
    class CustomFx
      def initialize(key)
        @key = key
      end

      def decode(data)
        "_" + data + "_"
      end
    end
  end
end

RSpec.describe 'ProcessCustom' do
  it 'parses test properly' do
    require 'process_custom'
    r = ProcessCustom.from_file('src/process_rotate.bin')

    expect(r.buf1).to eq [0x10, 0xb3, 0x94, 0x94, 0xf4].pack('C*')
    expect(r.buf2).to eq [0x5f, 0xba, 0x7b, 0x93, 0x63, 0x23, 0x5f].pack('C*')
    expect(r.buf3).to eq [0x29, 0x33, 0xb1, 0x38, 0xb1].pack('C*')
  end
end
