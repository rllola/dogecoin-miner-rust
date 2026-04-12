use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Dogecoin block header used in the test suite (test_scrypt_hasher)
const BLOCK_HEX: &str = "01000000f615f7ce3b4fc6b8f61e8f89aedb1d0852507650533a9e3b10b9bbcc30639f279fcaa86746e1ef52d3edb3c4ad8259920d509bd073605c9bf1d59983752a6b06b817bb4ea78e011d012d59d4";

// Dogecoin scrypt params: N=1024 (log2=10), r=1, p=1, output=32 bytes

fn bench_scrypt(c: &mut Criterion) {
    let block = hex::decode(BLOCK_HEX).unwrap();
    let params = scrypt::Params::new(10, 1, 1, 32).unwrap();

    c.bench_function("scrypt (RustCrypto)", |b| {
        b.iter(|| {
            let mut output = [0u8; 32];
            scrypt::scrypt(black_box(&block), black_box(&block), &params, &mut output).unwrap();
            output
        })
    });
}

criterion_group!(benches, bench_scrypt);
criterion_main!(benches);
