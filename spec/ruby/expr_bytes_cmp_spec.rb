# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ExprBytesCmp' do
  it 'parses test properly' do
    require 'expr_bytes_cmp'
    r = ExprBytesCmp.from_file('src/fixed_struct.bin')

    expect(r.one).to eq [80].pack('C*')
    expect(r.two).to eq [65, 67, 75].pack('C*')
    expect(r.is_eq).to eq true
    expect(r.is_ne).to eq false
    expect(r.is_lt).to eq true
    expect(r.is_gt).to eq false
    expect(r.is_le).to eq true
    expect(r.is_ge).to eq false
    expect(r.is_lt2).to eq false
    expect(r.is_gt2).to eq true
  end
end
