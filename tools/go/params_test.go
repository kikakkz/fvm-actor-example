package main

import (
	"bytes"
	"crypto/rand"
	"encoding/base64"
	"testing"

	"github.com/filecoin-project/go-address"
	"github.com/filecoin-project/go-state-types/abi"

	"github.com/filecoin-project/lotus/chain/actors"
	power6 "github.com/filecoin-project/specs-actors/v6/actors/builtin/power"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/peer"
)

// 相关约束
//
func TestCreateMiner(t *testing.T) {
	////////////////////////////// create miner
	// lotus-miner net id
	/*
		peerid, err := peer.Decode("12D3KooWELqUmx2JeUcX9e9VCPwPJdSxc9rXQLuLFELDeKdRiRiR")
		if err != nil {
			t.Error(err)
		}
		t.Logf("peer id %v", peerid)
	*/
	private, public, err := crypto.GenerateEd25519Key(rand.Reader)
	if err != nil {
		t.Error(err)
	}
	t.Logf("private %v public %v", private, public)

	peerid, err := peer.IDFromPrivateKey(private)
	if err != nil {
		t.Error(err)
	}
	t.Logf("peer id %v", peerid)

	// lotus-miner info
	owner, err := address.NewIDAddress(1000)
	if err != nil {
		t.Error(err)
	}
	t.Logf("owner %v", owner)

	cmp := &power6.CreateMinerParams{
		Owner:               owner,
		Worker:              owner,
		WindowPoStProofType: abi.RegisteredPoStProof_StackedDrgWindow2KiBV1,
		Peer:                abi.PeerID(peerid),
		Multiaddrs:          nil,
	}

	buf := bytes.Buffer{}
	if err := cmp.MarshalCBOR(&buf); err != nil {
		t.Error(err)
	}
	base64Ofcmp := base64.StdEncoding.EncodeToString(buf.Bytes())
	// this args used for lotus chain invoke
	t.Logf("create miner base64 args %v", base64Ofcmp)

	base64Ofcmpoffs, err := actors.SerializeParams(cmp)
	if err != nil {
		t.Error(err)
	}
	base64Ofcmpoff := base64.StdEncoding.EncodeToString(base64Ofcmpoffs)
	// this args used for lotus chain invoke
	t.Logf("create miner base64 args %v", base64Ofcmpoff)

	t.Logf("should equal %v == %v %v", base64Ofcmp, base64Ofcmpoff, base64Ofcmp == base64Ofcmpoff)
}

func TestChangeMinerOwner(t *testing.T) {
	////////////////////////////// change miner owner
	// lotus state list-actors
	contract, err := address.NewIDAddress(1001)
	if err != nil {
		t.Error(err)
	}

	t.Logf("contract addr %v", contract)
}

func TestChangeMinerWorker(t *testing.T) {
	////////////////////////////// change miner worker
}

func TestChangeMinerControl(t *testing.T) {
	////////////////////////////// change miner control
}
