RSpec.describe 'ValidFailEqBytes' do
  it 'parses test properly' do
    require 'valid_fail_eq_bytes'
    expect {
      r = ValidFailEqBytes.from_file('src/fixed_struct.bin')
    }.to raise_error(
      # Make extra sure that handling of "byte[]" is used by looking at the message.
      Kaitai::Struct::ValidationNotEqualError,
      a_string_including('[51 41]', '[50 41]')
    )
  end
end
