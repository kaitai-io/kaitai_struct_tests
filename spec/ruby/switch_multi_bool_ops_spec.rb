require 'switch_multi_bool_ops'

RSpec.describe SwitchMultiBoolOps do
  it 'parses test properly' do
    r = SwitchMultiBoolOps.from_file('src/switch_integers.bin')

    # The test just needs to compile/execute, we don't care about actual data.
  end
end
