RSpec.describe 'DebugEnumName' do
  it 'parses test properly' do
    require 'debug_enum_name'
    r = DebugEnumName.from_file('src/fixed_struct.bin')

    # --debug implies --no-auto-read
    r._read

    expect(r.one).to eq :test_enum1_enum_value_80
    expect(r.array_of_ints[0]).to eq :test_enum2_enum_value_65
    expect(r.test_type.field1).to eq :inner_enum1_enum_value_67
    expect(r.test_type.instance_field).to eq :inner_enum2_enum_value_11
  end
end
