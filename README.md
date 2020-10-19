# sst_dump_aggregator
Simple script that executed sst_dump from rocksdb on list of .sst files in folder and aggregates output.
You can find sst_dump and ldb tools [here](https://github.com/facebook/rocksdb/wiki/Administration-and-Data-Access-Tool)
call script with folder input like this:

`cargo run /home/openethereum/.local/share/openethereum_bigdb/chains/ethereum/db/906a34e69aec8c0d/overlayrecent/db/`


Output on fully synced prunning client, with  343GB database (got size of db from `du -hs *`) looks like this:
```
col:col0, data_blocks:0, entries:678 255 957,
                 raw_key_size:27 130 021 270, raw_value_size:64 359 960 576,
                 data_block_size:73 969 852 747, index_block_size:148 024 811, estimated_table_size:74 117 877 558
col:col1, data_blocks:0, entries:9 058 214,
                 raw_key_size:36 2328 560, raw_value_size:4 455 065 133,
                 data_block_size:3 747 570 420, index_block_size:7 265 966, estimated_table_size:3 754 836 386
col:col2, data_blocks:0, entries:9 058 216,
                 raw_key_size:362 328 640, raw_value_size:112 841 261 319,
                 data_block_size:80 255 637 166, index_block_size:97 681 821, estimated_table_size:80 353 318 987
col:col3, data_blocks:0, entries:638310069,
                 raw_key_size:2 591 7084 998, raw_value_size:279 951 569 212,
                 data_block_size:100 357 103 102, index_block_size:196 499 695, estimated_table_size:100 553 602 797
col:col4, data_blocks:0, entries:2,
                 raw_key_size:56, raw_value_size:4,
                 data_block_size:79, index_block_size:27, estimated_table_size:106
col:col6, data_blocks:0, entries:2,
                 raw_key_size:34, raw_value_size:4,
                 data_block_size:70, index_block_size:54, estimated_table_size:124
                 
```
