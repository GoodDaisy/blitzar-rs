extern crate curve25519_dalek;
extern crate pedersen;

use byte_slice_cast::AsByteSlice;
use curve25519_dalek::ristretto::CompressedRistretto;
use pedersen::compute::*;
use pedersen::sequences::*;

fn main() {
    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each
    // data vector is either a dense sequence or a sparse sequence.
    //
    // For instance, for the first commitment we have a dense sequence:
    //     commitment[0] = g[0]*dense_data[0] + g[1]*dense_data[1] + g[2]*dense_data[2] + g[3]^dense_data[3]
    //
    // Now for the second commitment we have a sparse sequence:
    //     commitment[0] = g[sparse_indices[0]]*sparse_data[0] +
    //                     g[sparse_indices[1]]*sparse_data[1] +
    //                     g[sparse_indices[2]]*sparse_data[2] +
    //                     g[sparse_indices[3]]^sparse_data[3]
    //
    // The `g` vector above is our generators. Its values are automatically generated by our library.
    /////////////////////////////////////////////
    let sparse_data: Vec<u32> = vec![1, 2, 3, 4, 9];
    let sparse_indices: Vec<u64> = vec![0, 2, 4, 5, 9];
    let dense_data: Vec<u32> = vec![1, 0, 2, 0, 3, 4, 0, 0, 0, 9, 0];

    /////////////////////////////////////////////
    // Fill the table with entries
    /////////////////////////////////////////////
    let table: Vec<Sequence> = vec![
        Sequence::Dense(DenseSequence {
            data_slice: dense_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&dense_data[0]),
        }),
        Sequence::Sparse(SparseSequence {
            data_slice: sparse_data.as_byte_slice(),
            element_size: std::mem::size_of_val(&sparse_data[0]),
            data_indices: &sparse_indices,
        }),
    ];

    /////////////////////////////////////////////
    // We need to define a commitment vector which
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0_u8; 32]); table.len()];

    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments(&mut commitments, &table);

    /////////////////////////////////////////////
    // We verify if the results were correctly computed
    /////////////////////////////////////////////
    if commitments[0] == commitments[1] {
        println!(
            "Sparse and Dense Commitment are equal: {:?}",
            commitments[0]
        );
    } else {
        println!("Sparse and Dense Commitment differ");
        println!("Dense Commitment: {:?}\n", commitments[0]);
        println!("Sparse Commitment: {:?}\n", commitments[1]);
    }
}
