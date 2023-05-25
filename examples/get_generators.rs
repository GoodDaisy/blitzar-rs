extern crate blitzar;
extern crate curve25519_dalek;

use blitzar::compute::*;
use blitzar::sequences::*;
use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};

fn main() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // will be translated into a single 32 bytes dalek CompressedRistretto data
    //
    // Note that you must specify the vector element type (u8, u16, u32, u64, u128)
    //
    // For instance:
    //     commitment[0] = gs[0]*data[4] + gs[1]*data[5] + gs[2]*data[6]
    //                   = gs[0]*4 + gs[1]*7 + gs[2]*6
    //
    // Those generators `gs` are automatically generated by our CPU/GPU code.
    // So we provide an interface to access them. We use the offset to get only
    // a subset of the generated used in the gpu/cpu code.
    //
    /////////////////////////////////////////////
    let data: Vec<u16> = vec![0, 0, 0, 0, 4, 7, 6, 0, 0, 0];

    /////////////////////////////////////////////
    // Fill the table with entries
    //
    // We need to wrapper the vector array inside the table object.
    // This object holds a slice of the data vector and the
    // total amount of bytes of each element stored in the vector
    /////////////////////////////////////////////
    table.push(Sequence::Dense(DenseSequence {
        data_slice: data.as_byte_slice(),
        element_size: std::mem::size_of_val(&data[0]),
    }));

    /////////////////////////////////////////////
    // randomly obtain the generator points
    // We want the generators from the [4, 6] index range
    /////////////////////////////////////////////
    let offset_generators: usize = 4;
    let generators_len = data.len() - offset_generators - 3;
    let mut gs = vec![RistrettoPoint::from_uniform_bytes(&[0_u8; 64]); generators_len];

    get_generators(&mut gs, offset_generators as u64);

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(&mut commitments, &table, 0_u64);

    /////////////////////////////////////////////
    // Use Dalek library to obtain the same
    // commitment that was computed in the GPU or
    // CPU above. Following, we randomly
    // obtain the generators
    /////////////////////////////////////////////
    let mut expected_commit = RistrettoPoint::from_uniform_bytes(&[0_u8; 64]);

    /////////////////////////////////////////////
    // Then we use the above generators `gs`,
    // as well as the data table as scalars
    // to verify that those generators `gs`
    // are indeed the ones used during the
    // commitment computation
    /////////////////////////////////////////////
    for i in 0..gs.len() {
        let mut scalar_bytes: [u8; 32] = [0; 32];
        scalar_bytes[0] = data[i + offset_generators] as u8;

        // Construct a Scalar by reducing a 256-bit little-endian integer modulo the group order ℓ.
        let ristretto_sc = curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar_bytes);

        let g_i = gs[i];

        expected_commit += ristretto_sc * g_i;
    }

    /////////////////////////////////////////////
    // Compare the Dalek and our CPU/GPU commitment
    /////////////////////////////////////////////
    println!("Computed Commitment: {:?}\n", commitments[0]);
    println!("Expected Commitment: {:?}\n", expected_commit.compress());
}
