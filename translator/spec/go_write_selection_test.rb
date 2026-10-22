require 'minitest/autorun'
require 'tmpdir'
require 'fileutils'
require 'open3'

class GoWriteSelectionTest < Minitest::Test
  ROOT = File.expand_path('../..', __dir__)

  def test_default_selection_reports_manual_templates_and_explicit_requests_still_fail
    Dir.mktmpdir do |dir|
      FileUtils.cp(File.join(ROOT, 'build-formats-go-write'), dir)
      File.write(File.join(dir, 'config'), "FORMATS_KSY_DIR=formats\nFORMATS_COMPILED_DIR=compiled\nCOMPILER_DIR=compiler\nFORMATS_REPO_DIR=formats\n")
      FileUtils.mkdir_p(File.join(dir, 'formats'))
      %w[fixed_contents str_encodings_escaping_enc str_encodings_escaping_to_s].each do |name|
        File.write(File.join(dir, 'formats', "#{name}.ksy"), '')
      end
      compiler = File.join(dir, 'compiler/jvm/target/universal/stage/bin/kaitai-struct-compiler')
      FileUtils.mkdir_p(File.dirname(compiler))
      File.write(compiler, <<~SH)
        #!/bin/sh
        for arg do
          echo "$arg"
          case "$arg" in *str_encodings_escaping*) exit 2 ;; esac
        done
      SH
      File.chmod(0755, compiler)
      File.write(File.join(dir, 'prepare-go'), "#!/bin/sh\nexit 0\n")
      File.chmod(0755, File.join(dir, 'prepare-go'))
      output, status = Open3.capture2e('./build-formats-go-write', chdir: dir)
      assert status.success?, output
      assert_includes output, 'negative_generation: formats/str_encodings_escaping_enc.ksy'
      assert_includes output, 'negative_generation: formats/str_encodings_escaping_to_s.ksy'
      assert_includes output.lines.map(&:chomp), 'formats/fixed_contents.ksy'
      refute_includes output.lines.map(&:chomp), 'formats/str_encodings_escaping_enc.ksy'
      refute_includes output.lines.map(&:chomp), 'formats/str_encodings_escaping_to_s.ksy'
      output, status = Open3.capture2e('./build-formats-go-write', 'formats/str_encodings_escaping_enc.ksy', chdir: dir)
      assert_equal 2, status.exitstatus, output
    end
  end
end
