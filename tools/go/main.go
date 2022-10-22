package main

import (
    "github.com/filecoin-project/lotus/chain/actors"
    cid "github.com/ipfs/go-cid"
    cbg "github.com/whyrusleeping/cbor-gen"
    "fmt"
    "encoding/base64"
    "io"
    "golang.org/x/xerrors"
)

type Cid struct {
    cid cid.Cid
}

func (t *Cid) MarshalCBOR(w io.Writer) error {
    if t == nil {
        _, err := w.Write(cbg.CborNull)
        return err
    }

    cw := cbg.NewCborWriter(w)

    if _, err := cw.Write([]byte{161}); err != nil {
        return err
    }

    // t.Cid (cid.Cid) (struct)
    if len("cid") > cbg.MaxLength {
        return xerrors.Errorf("Value in field \"Cid\" was too long")
    }

    if err := cw.WriteMajorTypeHeader(cbg.MajTextString, uint64(len("cid"))); err != nil {
        return err
    }
    if _, err := io.WriteString(w, string("cid")); err != nil {
        return err
    }

    if err := cbg.WriteCid(cw, t.cid); err != nil {
        return xerrors.Errorf("failed to write cid field t.cid: %w", err)
    }

    return nil
}

func (t *Cid) UnmarshalCBOR(r io.Reader) (err error) {
    *t = Cid{}

    cr := cbg.NewCborReader(r)

    maj, extra, err := cr.ReadHeader()
    if err != nil {
        return err
    }
    defer func() {
        if err == io.EOF {
            err = io.ErrUnexpectedEOF
        }
    }()

    if maj != cbg.MajMap {
        return fmt.Errorf("cbor input should be of type map")
    }

    if extra > cbg.MaxLength {
        return fmt.Errorf("cid: map struct too large (%d)", extra)
    }

    var name string
    n := extra

    for i := uint64(0); i < n; i++ {

        {
            sval, err := cbg.ReadString(cr)
            if err != nil {
                return err
            }

            name = string(sval)
        }

        switch name {
            // t.cid (cid.Cid) (struct)
        case "cid":

            {

                c, err := cbg.ReadCid(cr)
                if err != nil {
                    return xerrors.Errorf("failed to read cid field t.cid: %w", err)
                }

                t.cid = c

            }

        default:
            // Field doesn't exist on this type, so ignore it
            cbg.ScanForLinks(r, func(cid.Cid) {})
        }
    }

    return nil
}


func main() {
    _cid, err := cid.Decode("bafy2bzaceax3ounnbvdbkxa4divufisiz5ylmroka5gsfarg5nfnkfksdxmgq")
    if err != nil {
        fmt.Println(err)
	return
    }

    acid := &Cid {
        cid: _cid,
    }

    fmt.Println(acid)

    a, err := actors.SerializeParams(acid)
    if err != nil {
	fmt.Println(err)
        return
    }
    fmt.Println(a)
    fmt.Println(base64.StdEncoding.EncodeToString(a))

    a, _ = actors.SerializeParams(cbg.CborCid(_cid))
    fmt.Println(base64.StdEncoding.EncodeToString(a))
}

