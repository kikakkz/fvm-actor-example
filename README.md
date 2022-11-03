# 说明

## 涉及相关的仓库和需要的操作

1. [lotus](https://github.com/filecoin-project/lotus/tree/experimental/fvm-m2)
2. [builtin-actors](https://github.com/filecoin-project/builtin-actors/tree/next)
3. [ref-vm](https://github.com/filecoin-project/ref-fvm/tree/fvm%403.0.0-alpha.5)

+ builtin-actors
```sh
allinone: all-bundles bundle-wallaby bundle-devnet-wasm
	cd ./output && \
	tar -cf v8.tar.zst --use-compress-program zstd -- *.car && \
	cp -f v8.tar.zst ../../lotus/build/actors/v8.tar.zst && \
	cp -f builtin-actors-butterflynet.car ../../lotus/build/genesis/butterflynet.car && \
	cp -f builtin-actors-calibrationnet.car ../../lotus/build/genesis/calibnet.car && \
	cp -f builtin-actors-devnet-wasm.car ../../lotus/build/genesis/devnet.car && \
	cp -f builtin-actors-mainnet.car ../../lotus/build/genesis/mainnet.car && \
	rm ./*.car ./*.zst && \
	cd ../../lotus && \
	(make gen 2>/dev/null || make gen) && \
	git checkout chain/state && \
	make clean 2k && \
	cd ../builtin-actors
.PHONY: allinone
```

+ lotus
```sh
export LOTUS_USE_FVM_CUSTOM_BUNDLE=1
export LOTUS_VM_ENABLE_TRACING=1
```

```go
// chain/state/statetree.go
case network.Version13, network.Version14, network.Version15, network.Version16:=>case network.Version13, network.Version14, network.Version15, network.Version16, network.Version(18):
```
```sh
# 如果出错,修改api/proxy_gen.go 解决冲突, 重新编译
make gen
```

--------
## 相关的操作

### lotus

1. 查看当前节点所有的 **actors**

> lotus state list-actors

2. 查看当前节点所有的 **miner**

> lotus state list-miners

3. 查看 **miner** 信息

> lotus state miner-info miner-id

4. 产看当前所部署的 **builtin-actors** cid

> lotus state actor-cids

5. 查看 **actor** 信息

> lotus state get-actor actor-id

### chain operator

1. install actor

> lotus chain install-actor

2. create actor

> lotus chain create-actor cid

3. invoke actor

> lotus chain invoke actor-id method-id params

### lotus-miner

1. miner peer id

> lotus-miner net id

--------
## 需要实现的功能

+ 创建 miner
+ 变更 owner
+ 变更 woker
