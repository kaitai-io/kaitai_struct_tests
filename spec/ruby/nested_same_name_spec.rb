require 'nested_same_name'

RSpec.describe NestedSameName do
  it 'parses test properly' do
    r = NestedSameName.from_file('src/repeat_n_struct.bin')

    expect(r.main_data.main_size).to eq 2
    expect(r.main_data.foo.data).to eq [0x10, 0, 0, 0].pack('C*')
  end
end
