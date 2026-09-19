package specwrite

import (
	"bytes"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"math"
	"os"
	"reflect"
	"sort"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

type writeStruct interface {
	kaitai.Struct
	Check() error
	Write(*kaitai.Writer) error
}

type parseFunc func(*kaitai.Stream) (writeStruct, error)

func parseFile(filename string, parse parseFunc) (writeStruct, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	return parse(kaitai.NewStream(f))
}

func assertRoundTrip(t *testing.T, filename string, parse parseFunc) {
	t.Helper()
	// Keep the source in memory so lazy instances remain readable during dumping.
	source, err := os.ReadFile(filename)
	if err != nil {
		t.Fatal(err)
	}
	original, err := parse(kaitai.NewStream(bytes.NewReader(source)))
	if err != nil {
		t.Fatal(err)
	}
	before, err := dumpStruct(original)
	if err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(len(source))
	if err := original.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	afterStruct, err := parse(kaitai.NewStream(bytes.NewReader(buf.Bytes())))
	if err != nil {
		t.Fatal(err)
	}
	after, err := dumpStruct(afterStruct)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(before, after) {
		t.Fatalf("round-trip dump mismatch\nbefore: %#v\nafter:  %#v", before, after)
	}
}

func equalToFullFile(obj writeStruct, filename string) error {
	expected, err := os.ReadFile(filename)
	if err != nil {
		return err
	}
	if err := obj.Check(); err != nil {
		return err
	}
	buf := kaitai.NewFixedSeekableBuffer(len(expected))
	if err := obj.Write(kaitai.NewWriter(buf)); err != nil {
		return err
	}
	if !bytes.Equal(buf.Bytes(), expected) {
		return fmt.Errorf("serialized bytes differ\nexpected: %x\nactual:   %x", expected, buf.Bytes())
	}
	return nil
}

func assertEqualToFullFile(t *testing.T, obj writeStruct, filename string) {
	t.Helper()
	if err := equalToFullFile(obj, filename); err != nil {
		t.Fatal(err)
	}
}

func assertErrorIs(t *testing.T, err, target error) {
	t.Helper()
	if !errors.Is(err, target) {
		t.Errorf("expected errors.Is(%v, %v)", err, target)
	}
}

func assertErrorAs(t *testing.T, err error, target interface{}) {
	t.Helper()
	if !errors.As(err, target) {
		t.Errorf("expected error assignable to %T, got %v", target, err)
	}
}

// Only active ancestors are recorded: shared objects are dumped by value while
// actual cycles are represented by stable paths rather than pointer addresses.
type dumpIdentity struct {
	typeOf  reflect.Type
	pointer uintptr
	length  int
}

func dumpStruct(obj interface{}) (interface{}, error) {
	return dumpValue(reflect.ValueOf(obj), make(map[dumpIdentity]string), 50, "/")
}

var errorType = reflect.TypeOf((*error)(nil)).Elem()

func dumpValue(v reflect.Value, ancestors map[dumpIdentity]string, depth int, path string) (interface{}, error) {
	if depth < 0 {
		return nil, fmt.Errorf("%s: recursion depth limit reached", path)
	}
	if !v.IsValid() {
		return nil, nil
	}
	if v.Kind() == reflect.Interface {
		if v.IsNil() {
			return nil, nil
		}
		return dumpValue(v.Elem(), ancestors, depth, path)
	}
	switch v.Kind() {
	case reflect.Ptr, reflect.Map, reflect.Slice:
		if v.IsNil() {
			return nil, nil
		}
		id := dumpIdentity{typeOf: v.Type(), pointer: v.Pointer()}
		if v.Kind() == reflect.Slice {
			id.length = v.Len()
		}
		if ref, ok := ancestors[id]; ok {
			return map[string]interface{}{"$ref": ref}, nil
		}
		ancestors[id] = path
		defer delete(ancestors, id)
	}
	childPath := func(name string) string { return strings.TrimSuffix(path, "/") + "/" + name }
	if stream, ok := v.Interface().(*kaitai.Stream); ok {
		pos, err := stream.Pos()
		if err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		if _, err = stream.Seek(0, io.SeekStart); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		data, readErr := stream.ReadBytesFull()
		_, seekErr := stream.Seek(pos, io.SeekStart)
		if readErr != nil {
			return nil, fmt.Errorf("%s: %w", path, readErr)
		}
		if seekErr != nil {
			return nil, fmt.Errorf("%s: %w", path, seekErr)
		}
		return hex.EncodeToString(data), nil
	}
	if obj, ok := v.Interface().(writeStruct); ok {
		dump := make(map[string]interface{})
		for i := 0; i < v.NumMethod(); i++ {
			name := v.Type().Method(i).Name
			// Compiler metadata/control methods use a trailing underscore.
			if strings.HasSuffix(name, "_") || name == "Check" {
				continue
			}
			method := v.Method(i)
			typ := method.Type()
			if typ.NumIn() != 0 {
				continue
			}
			getter := strings.HasPrefix(name, "Get") && typ.NumOut() == 1
			instance := typ.NumOut() == 2 && typ.Out(1) == errorType
			if !getter && !instance {
				continue
			}
			result := method.Call(nil)
			if instance && !result[1].IsNil() {
				return nil, fmt.Errorf("%s: %w", childPath(name), result[1].Interface().(error))
			}
			value, err := dumpValue(result[0], ancestors, depth-1, childPath(name))
			if err != nil {
				return nil, err
			}
			dump[name] = value
		}
		// Descendants and lazy instances must be materialized before Check.
		if err := obj.Check(); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		return dump, nil
	}
	switch v.Kind() {
	case reflect.Ptr:
		return dumpValue(v.Elem(), ancestors, depth-1, path)
	case reflect.Slice, reflect.Array:
		if v.Type().Elem().Kind() == reflect.Uint8 {
			data := make([]byte, v.Len())
			for i := range data {
				data[i] = byte(v.Index(i).Uint())
			}
			return hex.EncodeToString(data), nil
		}
		items := make([]interface{}, v.Len())
		for i := range items {
			item, err := dumpValue(v.Index(i), ancestors, depth-1, childPath(fmt.Sprint(i)))
			if err != nil {
				return nil, err
			}
			items[i] = item
		}
		return items, nil
	case reflect.Map:
		keys := v.MapKeys()
		sort.Slice(keys, func(i, j int) bool { return fmt.Sprint(keys[i].Interface()) < fmt.Sprint(keys[j].Interface()) })
		items := make(map[string]interface{}, len(keys))
		for _, key := range keys {
			name := fmt.Sprint(key.Interface())
			item, err := dumpValue(v.MapIndex(key), ancestors, depth-1, childPath(name))
			if err != nil {
				return nil, err
			}
			items[name] = item
		}
		return items, nil
	case reflect.Float32, reflect.Float64:
		if math.IsNaN(v.Float()) {
			return "NaN", nil
		}
	}
	return v.Interface(), nil
}
