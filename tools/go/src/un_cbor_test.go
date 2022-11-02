package src_test

import (
	"bytes"
	"encoding/base64"
	"testing"

	power6 "github.com/filecoin-project/specs-actors/v6/actors/builtin/power"
	cbg "github.com/whyrusleeping/cbor-gen"
)

func TestUnCbor(t *testing.T) {
	cborBytes, err := base64.StdEncoding.DecodeString("hUMA+AZDAOcHCEMBAgOBQwECAw==")
	if err != nil {
		t.Error(err)
	}

	if err := cbg.ValidateCBOR(cborBytes); err != nil {
		t.Errorf("invalid cbor %v", err)
	}

	t.Log("decode cbor info", cborBytes)

	cdec := cbg.NewCborReader(bytes.NewReader(cborBytes))

	t.Log(cdec.ReadHeader())

	de := power6.CreateMinerParams{}
	if err := de.UnmarshalCBOR(bytes.NewReader(cborBytes)); err != nil {
		t.Error(err)
	}
	t.Log(de)
}
