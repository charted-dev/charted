//go:build (amd64 || arm64) && !purego

// Code generated by ./cmd/ch-gen-col, DO NOT EDIT.

package proto

import (
	"unsafe"

	"github.com/go-faster/errors"
)

// DecodeColumn decodes Enum8 rows from *Reader.
func (c *ColEnum8) DecodeColumn(r *Reader, rows int) error {
	if rows == 0 {
		return nil
	}
	*c = append(*c, make([]Enum8, rows)...)
	s := *(*slice)(unsafe.Pointer(c))
	dst := *(*[]byte)(unsafe.Pointer(&s))
	if err := r.ReadFull(dst); err != nil {
		return errors.Wrap(err, "read full")
	}
	return nil
}

// EncodeColumn encodes Enum8 rows to *Buffer.
func (c ColEnum8) EncodeColumn(b *Buffer) {
	v := c
	if len(v) == 0 {
		return
	}
	offset := len(b.Buf)
	b.Buf = append(b.Buf, make([]byte, len(v))...)
	s := *(*slice)(unsafe.Pointer(&v))
	src := *(*[]byte)(unsafe.Pointer(&s))
	dst := b.Buf[offset:]
	copy(dst, src)
}
