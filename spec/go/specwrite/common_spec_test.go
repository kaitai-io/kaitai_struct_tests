package specwrite

import (
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

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
