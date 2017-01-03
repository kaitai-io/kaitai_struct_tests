require 'debug_enum_name'

RSpec.describe DebugEnumName do
  it 'parses test properly' do
    r = DebugEnumName.from_file('src/fixed_struct.bin')
    r._read

    # this test is meaningful only for languages that have --debug and do
    # not save enum type info
  end
end
