require 'valid_fail_eq_str'

RSpec.describe ValidFailEqStr do
  it 'parses test properly' do
    expect {
      r = ValidFailEqStr.from_file('src/fixed_struct.bin')
    }.to raise_error(
      # Make extra sure that handling of "byte[]" is not used by looking at the message.
      Kaitai::Struct::ValidationNotEqualError,
      a_string_including('"BACK"', '"PACK"')
    )
  end
end
