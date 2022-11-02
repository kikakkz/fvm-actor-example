package main

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"

	"github.com/filecoin-project/lotus/chain/actors"
	power6 "github.com/filecoin-project/specs-actors/v6/actors/builtin/power"
	cid "github.com/ipfs/go-cid"
	cbg "github.com/whyrusleeping/cbor-gen"
	"golang.org/x/xerrors"

	// "github.com/filecoin-project/go-address"
	"crypto/rand"

	"github.com/filecoin-project/go-state-types/abi"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/peer"
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

	acid := &Cid{
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

	cmp := &power6.CreateMinerParams{}
	// b, err := base64.StdEncoding.DecodeString("hVgxA7ivFzAgasDgX6pDFcDTuw7qmQwCx/8JdFpJq3vi0zGQbSKIq8LCdHn2joqSuGwj9FgxA7ivFzAgasDgX6pDFcDTuw7qmQwCx/8JdFpJq3vi0zGQbSKIq8LCdHn2joqSuGwj9AdYJgAkCAESIMsrq9DjnCJpGRxh6dLHaCfPMMB0Fvuz/51vnaEIU7hFgA==")
	b, err := base64.StdEncoding.DecodeString("hVUCqk8SCpZRPRNry1+7ysNnbR6n2epYMQO4rxcwIGrA4F+qQxXA07sO6pkMAsf/CXRaSat74tMxkG0iiKvCwnR59o6KkrhsI/QIWCYAJAgBEiCt8gcOk3C7ZHa9WRlJzxawE/CBtUFik2VWWtXXtgXIGYA=")
	if err != nil {
		fmt.Println(err)
		return
	}
	buffer := bytes.NewReader(b)
	err = cmp.UnmarshalCBOR(buffer)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("======================================")
	b, _ = json.Marshal(cmp)
	fmt.Println(string(b))

	pk, _, err := crypto.GenerateEd25519Key(rand.Reader)
	if err != nil {
		fmt.Println(err)
		return
	}
	peerid, err := peer.IDFromPrivateKey(pk)
	if err != nil {
		fmt.Println(err)
		return
	}

	// peerid, _ = peer.Decode("12D3KooWMXNkWP1cZpsVjeKXnWwULFc6yhPAn1SYs2m3Fd2MK1LU")

	cmp = &power6.CreateMinerParams{
		WindowPoStProofType: abi.RegisteredPoStProof_StackedDrgWindow2KiBV1,
		Peer:                abi.PeerID(peerid),
		Multiaddrs:          nil,
	}
	fmt.Println("1 ********************************************")
	b, _ = json.Marshal(cmp)
	fmt.Println(string(b))

	a, err = actors.SerializeParams(cmp)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("2 ********************************************")
	fmt.Println(peerid)
	fmt.Println(string(abi.PeerID(peer.ID("12D3KooWRSHqnjxrj7RAp6wFTLgJUQEfGGnudAK51mATvzg2aFFV"))))
	b, _ = json.Marshal(abi.PeerID(peerid))
	fmt.Println(string(b))
	fmt.Println(base64.StdEncoding.EncodeToString(a))
	b, _ = json.Marshal(cmp)
	fmt.Println(string(b))

	peerid1, _ := peer.Decode("12D3KooWMXNkWP1cZpsVjeKXnWwULFc6yhPAn1SYs2m3Fd2MK1LU")
	fmt.Println(peerid1)
}
