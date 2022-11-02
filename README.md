# 说明

## 涉及相关的仓库和需要的操作

1. lotus
2. builtin-actors
3. ref-vm

+ builtin-actors

```toml
sector-default = ["sector-8m", "sector-64g", "sector-32g"]
```

```sh
all-bundles: bundle-mainnet bundle-caterpillarnet bundle-butterflynet bundle-calibrationnet bundle-devnet bundle-wallaby bundle-devnet-wasm bundle-testing

allinone:
	cd ./output && \
	tar -cf v8.tar.zst --use-compress-program "zstd -19" -- *.car && \
	cp v8.tar.zst ../../lotus/build/actors/v8.tar.zst && \
	cp builtin-actors-butterflynet.car ../../lotus/build/genesis/butterflynet.car && \
	cp builtin-actors-calibrationnet.car ../../lotus/build/genesis/calibnet.car && \
	cp builtin-actors-devnet-wasm.car ../../lotus/build/genesis/devnet.car && \
	cp builtin-actors-mainnet.car ../../lotus/build/genesis/mainnet.car && \
	cp builtin-actors-wallaby.car ../../lotus/build/genesis/wallabynet.car && \
	cd ../../lotus && \
	make gen && \
	cd ../builtin-actors

.PHONY: allinone
```

+ lotus
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

3. 产看当前所部署的 **builtin-actors** cid

> lotus state actor-cids

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
