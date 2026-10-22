package specwrite

import (
	"bytes"
	"reflect"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

// Literal vectors come from the KST assertions and their source files. These
// complement the generated round trips: inverse but incorrect codecs must fail.
func TestCustomProcessingExactBytes(t *testing.T) {
	for _, tc := range []struct {
		name    string
		source  []byte
		parse   parseFunc
		decoded func(writeStruct) [][]byte
		want    [][]byte
	}{
		{
			name:   "process_custom",
			source: []byte{9, 172, 141, 141, 237, 186, 123, 147, 99, 35, 1, 42, 52, 178, 57, 178},
			parse: func(s *kaitai.Stream) (writeStruct, error) {
				r := NewProcessCustom()
				return r, r.Read(s, r, r)
			},
			decoded: func(r writeStruct) [][]byte {
				p := r.(*ProcessCustom)
				return [][]byte{p.GetBuf1(), p.GetBuf2(), p.GetBuf3()}
			},
			want: [][]byte{{16, 179, 148, 148, 244}, {95, 186, 123, 147, 99, 35, 95}, {41, 51, 177, 56, 177}},
		},
		{
			name:   "process_custom_no_args",
			source: []byte{9, 172, 141, 141, 237},
			parse: func(s *kaitai.Stream) (writeStruct, error) {
				r := NewProcessCustomNoArgs()
				return r, r.Read(s, r, r)
			},
			decoded: func(r writeStruct) [][]byte { return [][]byte{r.(*ProcessCustomNoArgs).GetBuf()} },
			want:    [][]byte{{95, 9, 172, 141, 141, 237, 95}},
		},
		{
			name:   "process_term_struct",
			source: []byte("foo|bar|baz@"),
			parse: func(s *kaitai.Stream) (writeStruct, error) {
				r := NewProcessTermStruct()
				return r, r.Read(s, r, r)
			},
			decoded: func(r writeStruct) [][]byte {
				p := r.(*ProcessTermStruct)
				return [][]byte{p.GetS1().GetValue(), p.GetS2().GetValue(), p.GetS3().GetValue()}
			},
			want: [][]byte{[]byte("FOO"), []byte("BAR"), []byte("\\BAZ ")},
		},
	} {
		t.Run(tc.name, func(t *testing.T) {
			original := bytes.Clone(tc.source)
			r, err := tc.parse(kaitai.NewStream(bytes.NewReader(tc.source)))
			if err != nil {
				t.Fatal(err)
			}
			if got := tc.decoded(r); !reflect.DeepEqual(got, tc.want) {
				t.Fatalf("decoded = %x; want %x", got, tc.want)
			}
			// The shared dumper checks descendants before their parent.
			if _, err := dumpStruct(r); err != nil {
				t.Fatal(err)
			}
			buf := kaitai.NewFixedSeekableBuffer(len(original))
			if err := r.Write(kaitai.NewWriter(buf)); err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(buf.Bytes(), original) {
				t.Fatalf("serialized = %x; want %x", buf.Bytes(), original)
			}
			reread, err := tc.parse(kaitai.NewStream(bytes.NewReader(buf.Bytes())))
			if err != nil {
				t.Fatal(err)
			}
			if got := tc.decoded(reread); !reflect.DeepEqual(got, tc.want) {
				t.Fatalf("reparsed = %x; want %x", got, tc.want)
			}
			if !bytes.Equal(tc.source, original) {
				t.Fatal("parse or write mutated source bytes")
			}
		})
	}
}
