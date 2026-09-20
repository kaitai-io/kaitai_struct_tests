require 'minitest/autorun'
require 'open3'

# The production handwritten files are the source of truth. Regeneration into
# the default output directory must retain them, including negative templates.
class GoWriteManualTest < Minitest::Test
  ROOT = File.expand_path('../..', __dir__)
  FIXTURES = %w[cast_to_top expr_io_ternary expr_to_i_trailing debug_array_user_eof_exception debug_array_user_current_excluded str_encodings_escaping_enc str_encodings_escaping_to_s].freeze

  def test_regeneration_preserves_handwritten_contracts
    output, status = Open3.capture2e(ENV.fetch('SBT', 'sbt'), "run -t go-write #{FIXTURES.join(' ')}", chdir: File.join(ROOT, 'translator'))
    assert status.success?, output
    FIXTURES.each do |fixture|
      manual = File.binread(File.join(ROOT, 'spec/go/specwrite', "#{fixture}_test.go"))
      generated = File.binread(File.join(ROOT, 'spec/ks/out/go/specwrite', "#{fixture}_test.go"))
      assert_equal manual, generated, "#{fixture}: regeneration must preserve the handwritten contract"
    end
    before = FIXTURES.to_h { |fixture| [fixture, File.binread(File.join(ROOT, 'spec/go/specwrite', "#{fixture}_test.go"))] }
    output, status = Open3.capture2e(ENV.fetch('SBT', 'sbt'), "run -t go-write -f #{FIXTURES.join(' ')}", chdir: File.join(ROOT, 'translator'))
    assert status.success?, output
    before.each do |fixture, contents|
      assert_equal contents, File.binread(File.join(ROOT, 'spec/go/specwrite', "#{fixture}_test.go")), "#{fixture}: --force must preserve the handwritten contract"
    end
  end
end
