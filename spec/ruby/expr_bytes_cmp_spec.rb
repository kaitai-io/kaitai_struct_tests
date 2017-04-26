require 'expr_bytes_cmp'

RSpec.describe ExprBytesCmp do
  it 'parses test properly' do
    r = ExprBytesCmp.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 'P'
    expect(r.two).to eq 'ACK'

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
