RSpec.describe 'OpaqueExternalType02Parent' do
  it 'parses test properly' do
    require 'opaque_external_type_02_parent'
    require 'opaque_external_type_02_child' # the opaque type
    r = OpaqueExternalType02Parent.from_file('src/term_strz.bin')

    expect(r.parent.child.s1).to eq "foo"
    expect(r.parent.child.s2).to eq "bar"
    expect(r.parent.child.s3.s3).to eq "|baz@"
  end
end
