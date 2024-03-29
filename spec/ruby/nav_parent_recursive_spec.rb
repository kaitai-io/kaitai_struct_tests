# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'NavParentRecursive' do
  it 'parses test properly' do
    require 'nav_parent_recursive'
    r = NavParentRecursive.from_file('src/enum_negative.bin')

    expect(r.value).to eq 255
    expect(r.next.value).to eq 1
    expect(r.next.parent_value).to eq 255
    expect(r.next.next).to be_nil
  end
end
