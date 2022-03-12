# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'bits_signed_shift_b32_le'
rescue SyntaxError => e
  load_err = e
  BitsSignedShiftB32Le = nil
rescue LoadError => e
  load_err = e
  BitsSignedShiftB32Le = nil
end

RSpec.describe BitsSignedShiftB32Le do
  it 'parses test properly' do
    raise load_err if BitsSignedShiftB32Le.nil?
    r = BitsSignedShiftB32Le.from_file('src/bits_signed_shift_b32_le.bin')

    expect(r.a).to eq 0
    expect(r.b).to eq 255
  end
end