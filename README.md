# Goerli ETH Aidrdrop

## Methodology
1. Fetch all addresses which have deployed on mainnet before the merge
Ran a rust script colocated with a geth node to do this in ~4hrs. For some reason it returned a lot of false positives. Script fetched all transactions from all blocks and stored counters for each EOA that sent a tx with no `to` address (i.e. contract creations).

2. Filter out false positives
Ran the data through a filter step which ensured they had a nonzero tx nonce. This seems to have removed all false positives

## Data
The process above found 292,979 EOAs which had deployed contracts on mainnet before block 15537393. Most of these only deployed a single contract.

## Distribution
My proposed distribution is as follows:
- 1 deployment (192359 addresses): 10 goETH
- 2-9 deployments (89014 addresses): 100 goETH
- 10+ deployments (11606 addresses): 1000 goETH

This would requier a total of 22,430,990 goETH distributed.   

The distribution is done through a forge script which transfers goETH through a multi-send contract in 1000 transfer chunks

### How to run distributor

```bash
cd distributor
PRIVATE_KEY=key forge script ./script/Distributor.s.sol
```
