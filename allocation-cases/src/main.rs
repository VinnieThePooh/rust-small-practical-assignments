use crate::api_demos::{boxed_slices, clone_from_may_avoid_allocations_demo, cow_usage_samples, indexing_over_slice_not_vec, iterator_chain_demo};

mod api_demos;
mod type_sizes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // clone_from_may_avoid_allocations_demo();
    // cow_usage_samples();
    // boxed_slices();
    // iterator_chain_demo()?;
    indexing_over_slice_not_vec();
    Ok(())
}
