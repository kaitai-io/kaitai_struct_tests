package specwrite

import (
	"bytes"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestOpaqueExternalType02Parent(t *testing.T) {
	want := []byte("foo|bar|baz@")
	parsed := NewOpaqueExternalType02Parent()
	if err := parsed.Read(kaitai.NewStream(bytes.NewReader(want)), nil, parsed); err != nil {
		t.Fatal(err)
	}
	parsedChild := parsed.GetParent().GetChild()
	if parsedChild.Parent_() != nil || parsedChild.GetS3().Parent_() != parsedChild {
		t.Fatal("parsed opaque child topology is incorrect")
	}
	if parsedChild.GetS1() != "foo" || parsedChild.GetS2() != "bar" || parsedChild.GetS3().GetS3() != "|baz@" {
		t.Fatalf("parsed values = %q %q %q", parsedChild.GetS1(), parsedChild.GetS2(), parsedChild.GetS3().GetS3())
	}

	r := NewOpaqueExternalType02Parent()
	r.SetRoot_(r)
	parent := NewOpaqueExternalType02Parent_ParentObj()
	parent.SetParent_(r)
	parent.SetRoot_(r)
	child := NewOpaqueExternalType02Child()
	child.SetRoot_(child)
	child.SetParent_(nil)
	child.SetS1("foo")
	child.SetS2("bar")
	childS3 := NewOpaqueExternalType02Child_OpaqueExternalType02ChildChild()
	childS3.SetParent_(child)
	childS3.SetRoot_(child)
	childS3.SetS3("|baz@")
	if err := childS3.Check(); err != nil {
		t.Fatal(err)
	}
	child.SetS3(childS3)
	if err := child.Check(); err != nil {
		t.Fatal(err)
	}
	if child.Parent_() != nil || childS3.Parent_() != child {
		t.Fatal("fresh opaque child topology is incorrect")
	}
	parent.SetChild(child)
	if err := parent.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetParent(parent)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(len(want))
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(buf.Bytes(), want) {
		t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
	}
	reparsed := NewOpaqueExternalType02Parent()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	if got := reparsed.GetParent().GetChild(); got.GetS1() != "foo" || got.GetS2() != "bar" || got.GetS3().GetS3() != "|baz@" || got.Parent_() != nil {
		t.Fatal("opaque external type did not reparse with its independent topology")
	}
}
