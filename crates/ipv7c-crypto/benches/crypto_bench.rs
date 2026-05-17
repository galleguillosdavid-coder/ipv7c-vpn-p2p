use criterion::{criterion_group, criterion_main, Criterion};
use ipv7c_crypto::aead;

fn bench_encrypt(c: &mut Criterion) {
    let key = [42u8; 32];
    let nonce = aead::generate_nonce();
    let data = vec![0u8; 1400]; // typical MTU-sized packet
    c.bench_function("chacha20_encrypt_1400b", |b| {
        b.iter(|| aead::encrypt(&key, &nonce, &data, b"ipv7c"))
    });
}

fn bench_decrypt(c: &mut Criterion) {
    let key = [42u8; 32];
    let nonce = aead::generate_nonce();
    let data = vec![0u8; 1400];
    let ct = aead::encrypt(&key, &nonce, &data, b"ipv7c").unwrap();
    c.bench_function("chacha20_decrypt_1400b", |b| {
        b.iter(|| aead::decrypt(&key, &nonce, &ct, b"ipv7c"))
    });
}

criterion_group!(benches, bench_encrypt, bench_decrypt);
criterion_main!(benches);
