```sh
cargo clean
```

mode 64 bit  
```sh
cargo +nightly build -p blotos --target x86_64-blotos.json -Z build-std=core,compiler_builtins,alloc -Z build-std-features=compiler-builtins-mem
```


```sh
cargo run -p xtask
```

```shell
qemu-system-x86_64 -drive format=raw,file=target/bios.img -serial stdio -no-reboot -d int 2>qemu_int.log
```