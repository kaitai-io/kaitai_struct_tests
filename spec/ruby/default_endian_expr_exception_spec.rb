require 'default_endian_expr_exception'

RSpec.describe DefaultEndianExprException do
  it 'parses test properly' do
    expect {
      r = DefaultEndianExprException.from_file('src/endian_expr.bin')
    }.to raise_error(Kaitai::Struct::Stream::UndecidedEndiannessError)
  end
end
