package specwrite

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// The two source KSTs explicitly describe manual negative-test templates, not
// successful roundtrip inputs. Isolate each source spelling so rejecting the
// first codec cannot conceal escaping defects in the other three.
func assertInvalidEncodingDiagnostics(t *testing.T, fixture string, expressions bool) {
	t.Helper()
	source, err := os.ReadFile("../../../formats/" + fixture + ".ksy")
	if err != nil {
		t.Fatal(err)
	}
	var literals []string
	for _, line := range strings.Split(string(source), "\n") {
		line = strings.TrimSpace(line)
		if expressions && strings.HasPrefix(line, "str") && strings.Contains(line, "_raw.to_s(") {
			literals = append(literals, line[strings.Index(line, ".to_s("):])
		} else if !expressions && strings.HasPrefix(line, "encoding:") {
			literals = append(literals, strings.TrimPrefix(line, "encoding:"))
		}
	}
	want := []string{`ASCII\\x`, `UTF-8\'x`, `SJIS\"x`, `IBM437\nx`}
	if len(literals) != len(want) {
		t.Fatalf("found %d source encodings, want %d", len(literals), len(want))
	}
	compiler := os.Getenv("KSC")
	if compiler == "" {
		compiler = "../../../../compiler/jvm/target/universal/stage/bin/kaitai-struct-compiler"
	}
	for _, mode := range []string{"read", "read_write"} {
		for i, literal := range literals {
			t.Run(fmt.Sprintf("negative_generation/%s/str%d", mode, i+1), func(t *testing.T) {
				dir := t.TempDir()
				schema := "meta:\n  id: invalid_encoding\n"
				if expressions {
					schema += "seq:\n  - id: raw\n    size: 0\ninstances:\n  text:\n    value: |\n      raw" + literal + "\n"
				} else {
					schema += "seq:\n  - id: text\n    size: 0\n    type: str\n    encoding:" + literal + "\n"
				}
				path := filepath.Join(dir, "invalid_encoding.ksy")
				if err := os.WriteFile(path, []byte(schema), 0600); err != nil {
					t.Fatal(err)
				}
				args := []string{"--", "-t", "go", "-d", dir}
				if mode == "read_write" {
					args = append(args, "--read-write")
				}
				args = append(args, path)
				ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
				defer cancel()
				output, err := exec.CommandContext(ctx, compiler, args...).CombinedOutput()
				if err == nil {
					t.Fatalf("invalid codec unexpectedly compiled: %s", output)
				}
				if exit, ok := err.(*exec.ExitError); !ok || exit.ExitCode() != 2 {
					t.Fatalf("expected compiler rejection (2), got %v: %s", err, output)
				}
				diagnostic := "encoding '" + want[i] + "' in not supported in Go"
				if !strings.Contains(string(output), diagnostic) {
					t.Fatalf("diagnostic did not preserve %q:\n%s", diagnostic, output)
				}
				if _, err := os.Stat(filepath.Join(dir, "invalid_encoding.go")); !os.IsNotExist(err) {
					t.Fatalf("rejected schema emitted Go source: %v", err)
				}
			})
		}
	}
}
