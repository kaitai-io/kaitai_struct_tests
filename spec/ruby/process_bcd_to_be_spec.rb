require 'process_bcd_to_be'

RSpec.describe ProcessBcdToBe do
  it 'parses test properly' do
    r = ProcessBcdToBe.from_file('src/process_bcd_to_be.bin')

    expect(r.ltr.value).to eq "12345678"
    expect(r.rtl.value).to eq "78563412"
    expect(r.decimal.value).to eq 12345678
  end
end
