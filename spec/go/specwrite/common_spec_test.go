package specwrite

import (
	"bytes"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

func TestHarnessStreamDumpPreservesPendingBits(t *testing.T) {
	for _, littleEndian := range []bool{false, true} {
		t.Run(fmt.Sprintf("littleEndian=%v", littleEndian), func(t *testing.T) {
			stream := kaitai.NewStream(bytes.NewReader([]byte{0xab, 0xcd}))
			readBits := stream.ReadBitsIntBe
			wantFirst, wantRemaining := uint64(5), uint64(11)
			if littleEndian {
				readBits = stream.ReadBitsIntLe
				wantFirst, wantRemaining = 3, 21
			}
			if got, err := readBits(3); err != nil || got != wantFirst {
				t.Fatalf("first bits: %d, %v", got, err)
			}
			for i := 0; i < 2; i++ {
				dump, err := dumpStruct(stream)
				if err != nil || dump != "abcd" {
					t.Fatalf("full stream dump: %v, %v", dump, err)
				}
				if pos, err := stream.ReadSeeker.Seek(0, io.SeekCurrent); err != nil || pos != 1 {
					t.Fatalf("raw position changed: %d, %v", pos, err)
				}
			}
			if got, err := readBits(5); err != nil || got != wantRemaining {
				t.Fatalf("remaining buffered bits: got %d, want %d, error %v", got, wantRemaining, err)
			}
			if got, err := stream.ReadU1(); err != nil || got != 0xcd {
				t.Fatalf("next byte: %x, %v", got, err)
			}
		})
	}
}

type dumpFaultReader struct {
	io.ReadSeeker
	seekCalls int
	failSeek  map[int]error
	readErr   error
}

func (r *dumpFaultReader) Seek(offset int64, whence int) (int64, error) {
	r.seekCalls++
	// Even a failed seek may have changed the underlying position.
	pos, err := r.ReadSeeker.Seek(offset, whence)
	if failure := r.failSeek[r.seekCalls]; failure != nil {
		return pos, failure
	}
	return pos, err
}

func (r *dumpFaultReader) Read(p []byte) (int, error) {
	if r.readErr != nil {
		return 0, r.readErr
	}
	return r.ReadSeeker.Read(p)
}

func TestHarnessStreamDumpFailuresRestorePositionAndPreserveBits(t *testing.T) {
	positionErr := errors.New("position failure")
	seekErr := errors.New("initial seek failure")
	readErr := errors.New("read failure")
	restoreErr := errors.New("restore failure")
	for _, tc := range []struct {
		name       string
		seekErrors map[int]error
		readError  error
		want       []error
		seekCalls  int
	}{
		{"position", map[int]error{1: positionErr}, nil, []error{positionErr}, 1},
		{"seek", map[int]error{2: seekErr}, nil, []error{seekErr}, 3},
		{"read", nil, readErr, []error{readErr}, 3},
		{"restore", map[int]error{3: restoreErr}, nil, []error{restoreErr}, 3},
		{"read and restore", map[int]error{3: restoreErr}, readErr, []error{restoreErr, readErr}, 3},
		{"seek and restore", map[int]error{2: seekErr, 3: restoreErr}, nil, []error{restoreErr, seekErr}, 3},
	} {
		t.Run(tc.name, func(t *testing.T) {
			raw := bytes.NewReader([]byte{0xab, 0xcd})
			stream := kaitai.NewStream(raw)
			if _, err := stream.ReadBitsIntBe(3); err != nil {
				t.Fatal(err)
			}
			faults := &dumpFaultReader{ReadSeeker: raw, failSeek: tc.seekErrors, readErr: tc.readError}
			stream.ReadSeeker = faults
			_, err := dumpStruct(stream)
			for _, want := range tc.want {
				if !errors.Is(err, want) {
					t.Fatalf("lost cause %v: %v", want, err)
				}
			}
			if faults.seekCalls != tc.seekCalls {
				t.Fatalf("seek count: got %d, want %d", faults.seekCalls, tc.seekCalls)
			}
			if pos, _ := raw.Seek(0, io.SeekCurrent); pos != 1 {
				t.Fatalf("position was not restored: %d", pos)
			}
			if got, bitErr := stream.ReadBitsIntBe(5); bitErr != nil || got != 11 {
				t.Fatalf("buffered bits changed on failure: %d, %v", got, bitErr)
			}
			if errors.Is(err, restoreErr) && !strings.HasPrefix(err.Error(), "/: restore stream position:") {
				t.Fatalf("restoration failure must take priority: %v", err)
			}
		})
	}
}

type harnessNode struct {
	child    *harnessNode
	checked  bool
	lazy     bool
	value    byte
	checkErr error
}

func (n *harnessNode) IO_() *kaitai.Stream                { panic("internal metadata traversed") }
func (n *harnessNode) Parent_() *harnessNode              { panic("parent traversed") }
func (n *harnessNode) ParamWriter_() *kaitai.Writer       { panic("writer traversed") }
func (n *harnessNode) WriteNoAlign_(*kaitai.Writer) error { panic("lifecycle invoked") }
func (n *harnessNode) GetValue() byte                     { return n.value }
func (n *harnessNode) Child() (*harnessNode, error) {
	n.lazy = true
	return n.child, nil
}
func (n *harnessNode) Check() error {
	if !n.lazy {
		return errors.New("Check before lazy instance")
	}
	if n.child != nil && n.child != n && !n.child.checked {
		return errors.New("parent before child")
	}
	n.checked = true
	return n.checkErr
}
func (n *harnessNode) Write(w *kaitai.Writer) error {
	if !n.checked {
		return &kaitai.ConsistencyNotCheckedError{}
	}
	return w.WriteU1(n.value)
}

func TestHarnessDumpMaterializesChecksAndHandlesCycles(t *testing.T) {
	child := &harnessNode{value: 2}
	parent := &harnessNode{value: 1, child: child}
	dump, err := dumpStruct(parent)
	if err != nil {
		t.Fatal(err)
	}
	if !parent.checked || !child.checked {
		t.Fatal("objects were not checked")
	}
	if strings.Contains(fmt.Sprint(dump), "Writer") {
		t.Fatalf("internal plumbing: %v", dump)
	}
	parent.child = parent
	dump, err = dumpStruct(parent)
	if err != nil || !strings.Contains(fmt.Sprint(dump), "$ref:/") {
		t.Fatalf("cycle: %v, %v", dump, err)
	}
	m := map[string]interface{}{}
	m["self"] = m
	s := make([]interface{}, 1)
	s[0] = s
	for _, v := range []interface{}{m, s} {
		if _, err := dumpStruct(v); err != nil {
			t.Fatal(err)
		}
	}
}

func TestHarnessFailuresRemainTyped(t *testing.T) {
	cause := &kaitai.ConsistencyNotCheckedError{}
	_, err := dumpStruct(&harnessNode{checkErr: cause})
	var typed *kaitai.ConsistencyNotCheckedError
	if !errors.Is(err, cause) || !errors.As(err, &typed) {
		t.Fatalf("lost cause: %v", err)
	}
	assertErrorIs(t, fmt.Errorf("wrapped: %w", io.ErrClosedPipe), io.ErrClosedPipe)
	assertErrorAs(t, err, &typed)
}

func TestHarnessExactFile(t *testing.T) {
	path := filepath.Join(t.TempDir(), "expected.bin")
	if err := os.WriteFile(path, []byte{42}, 0600); err != nil {
		t.Fatal(err)
	}
	n := &harnessNode{value: 42, lazy: true}
	assertEqualToFullFile(t, n, path)
	n.value = 43
	if err := equalToFullFile(n, path); err == nil || !strings.Contains(err.Error(), "2a") || !strings.Contains(err.Error(), "2b") {
		t.Fatalf("expected diagnostic hex mismatch: %v", err)
	}
}
