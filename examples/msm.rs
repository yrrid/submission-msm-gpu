use std::time::Instant;

use ark_bls12_377::G1Affine;
use ark_ff::BigInteger256;
use blst_msm::{
    multi_scalar_mult, multi_scalar_mult_init,
    util::{self, generate_nonuniform_scalars},
};

/// cargo run --release --example msm
fn main() {
    let bench_npow: usize = std::env::var("BENCH_NPOW")
        .unwrap_or("23".to_string())
        .parse()
        .unwrap();
    let npoints_npow = bench_npow;

    let batches = 1;
    let (points, scalars) =
        util::generate_points_scalars::<G1Affine>(1usize << npoints_npow, batches);

    let mut context = multi_scalar_mult_init(points.as_slice());

    let start = Instant::now();
    let res = multi_scalar_mult(&mut context, &points.as_slice(), unsafe {
        std::mem::transmute::<&[_], &[BigInteger256]>(scalars.as_slice())
    });
    println!("random time: {:?}", start.elapsed());

    let nonuniform_scalars = generate_nonuniform_scalars::<G1Affine>(1 << npoints_npow);
    let start = Instant::now();
    let res = multi_scalar_mult(&mut context, &points.as_slice(), unsafe {
        std::mem::transmute::<&[_], &[BigInteger256]>(nonuniform_scalars.as_slice())
    });
    println!("nonuniform time: {:?}", start.elapsed());
}
