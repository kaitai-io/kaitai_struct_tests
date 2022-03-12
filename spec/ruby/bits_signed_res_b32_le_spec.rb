# Autogenerated from KST: please remove this line if doing any edits by hand!

load_err = nil
begin
  require 'bits_signed_res_b32_le'
rescue SyntaxError => e
  load_err = e
  BitsSignedResB32Le = nil
rescue LoadError => e
  load_err = e
  BitsSignedResB32Le = nil
end

RSpec.describe BitsSignedResB32Le do
  it 'parses test properly' do
    raise load_err if BitsSignedResB32Le.nil?
    r = BitsSignedResB32Le.from_file('src/bits_shift_by_b32_le.bin')

    expect(r.a).to eq 4294967295
  end
end