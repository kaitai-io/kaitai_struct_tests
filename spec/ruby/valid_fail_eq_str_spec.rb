RSpec.describe 'ValidFailEqStr' do
  it 'parses test properly' do
    require 'valid_fail_eq_str'
    expect {
      r = ValidFailEqStr.from_file('src/fixed_struct.bin')
    }.to raise_error(
      # Make extra sure that handling of "byte[]" is NOT used by looking at the message.
      Kaitai::Struct::ValidationNotEqualError,
      a_string_including('"BACK"', '"PACK"')
    )
  end
end
