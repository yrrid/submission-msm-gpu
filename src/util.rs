// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_std::UniformRand;
use ark_std::{One, Zero};

pub fn generate_points_scalars<G: AffineCurve>(
    len: usize,
    batches: usize,
) -> (Vec<G>, Vec<G::ScalarField>) {
    let rand_gen: usize = 1 << 15;
    let mut rng = ChaCha20Rng::from_entropy();

    let mut points = <G::Projective as ProjectiveCurve>::batch_normalization_into_affine(
        &(0..rand_gen)
            .map(|_| G::Projective::rand(&mut rng))
            .collect::<Vec<_>>(),
    );
    // Sprinkle in some infinity points
    // points[3] = G::zero();
    while points.len() < len {
        points.append(&mut points.clone());
    }

    let scalars = (0..len * batches)
        .map(|_| G::ScalarField::rand(&mut rng))
        .collect::<Vec<_>>();

    (points, scalars)
}

/// Generate non-uniform scalars. For now we model after the lurk witness
/// which has ~20k over-represented values that occur ~300 times on average,
/// but can go up to ~1000s.
///
/// Hence we do:
/// - A few (<10) small values that occur at very high frequency,
///   mostly to represent special values like 0,1,2^32,2^64
/// - A few hundred random values that occur in the thousands
/// - ~20k values that occur between 200-400 times
/// - fill remaining length with random values
pub fn generate_nonuniform_scalars<G: AffineCurve>(len: usize) -> Vec<G::ScalarField> {
    assert!(len >= (1 << 23)); // fine tuned for this number lmao

    let mut rng = ChaCha20Rng::from_entropy();
    let mut scalars = Vec::with_capacity(len);

    // special values 200_000
    for _ in 0..150_000 {
        scalars.push(G::ScalarField::zero());
    }
    for _ in 0..50_000 {
        scalars.push(G::ScalarField::one());
    }

    // high freq: 250 * 6000 = 1_500_000
    let n_high_freq = rng.gen_range(200..300);
    let high_freq = (0..n_high_freq)
        .map(|_| G::ScalarField::rand(&mut rng))
        .collect::<Vec<_>>();

    for val in high_freq {
        let freq = rng.gen_range(4000..8000);
        for _ in 0..freq {
            scalars.push(val);
        }
    }

    // low freq: 20_000 * 300 = 3_000_000
    let n_low_freq = rng.gen_range(19_000..21_000);
    let low_freq = (0..n_low_freq)
        .map(|_| G::ScalarField::rand(&mut rng))
        .collect::<Vec<_>>();

    for val in low_freq {
        let freq = rng.gen_range(200..400);
        for _ in 0..freq {
            scalars.push(val);
        }
    }

    let n_rest = len - scalars.len();
    for _ in 0..n_rest {
        scalars.push(G::ScalarField::rand(&mut rng));
    }

    scalars.shuffle(&mut rng);

    assert_eq!(scalars.len(), len);
    scalars
}
