require 'js_signed_right_shift'

RSpec.describe JsSignedRightShift do
  it 'parses test properly' do
    r = JsSignedRightShift.from_file('src/fixed_struct.bin')

    expect(r.should_be_40000000).to eq 0x40000000
    expect(r.should_be_a00000).to eq 0xa00000
  end
end
