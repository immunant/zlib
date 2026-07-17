# zlib Cross Matrix Test Driver

Run a practical sampled matrix:
```sh
$ ./cross-zlib-matrix.py ./canterbury-corpus/canterbury \
    --target-driver ./zapi-rust \
    --reference-driver ./zapi-reference \
    --sample-configs 200 \
```

Sampling is deterministic; change it with `--seed`:
```sh
$ ./cross-zlib-matrix.py ./canterbury-corpus/canterbury \
    --target-driver ./zapi-rust \
    --reference-driver ./zapi-reference \
    --sample-configs 500 \
    --seed 42
```

## Run the complete cross-product

Omit `--sample-configs`:
```sh
$ ./cross-zlib-matrix.py ./canterbury-corpus/canterbury \
    --target-driver ./zapi-rust \
    --reference-driver ./zapi-reference \
```

The default full matrix has 3,600 compression configurations per implementation per corpus file.
Each compressed result is decompressed by both implementations with each decompressor buffer configuration, so a complete Canterbury run is intentionally very expensive.

A more manageable complete run can restrict one dimension:
```sh
$ ./cross-zlib-matrix.py ./canterbury-corpus/canterbury \
    --target-driver ./zapi-rust \
    --reference-driver ./zapi-reference \
    --levels 0,1,6,9 \
    --strategies default,fixed \
    --mem-levels 1,8,9 \
    --formats raw,zlib,gzip
```
