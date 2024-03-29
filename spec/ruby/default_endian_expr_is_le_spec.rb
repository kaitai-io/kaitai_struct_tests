# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'DefaultEndianExprIsLe' do
  it 'parses test properly' do
    require 'default_endian_expr_is_le'
    r = DefaultEndianExprIsLe.from_file('src/endian_expr.bin')

    expect(r.docs[0].indicator).to eq [73, 73].pack('C*')
    expect(r.docs[0].main.some_int).to eq 66
    expect(r.docs[0].main.some_int_be).to eq 66
    expect(r.docs[0].main.some_int_le).to eq 66
    expect(r.docs[1].indicator).to eq [77, 77].pack('C*')
    expect(r.docs[1].main.some_int).to eq 66
    expect(r.docs[1].main.some_int_be).to eq 66
    expect(r.docs[1].main.some_int_le).to eq 66
    expect(r.docs[2].indicator).to eq [88, 88].pack('C*')
    expect(r.docs[2].main.some_int).to eq 66
    expect(r.docs[2].main.some_int_be).to eq 66
    expect(r.docs[2].main.some_int_le).to eq 66
  end
end
