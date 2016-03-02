# coding: utf-8
require 'instance_std'

RSpec.describe InstanceStd do
  it 'parses test properly' do
    r = InstanceStd.from_file('src/str_encodings.bin')

    expect(r.header).to eq 'Some '
  end
end
