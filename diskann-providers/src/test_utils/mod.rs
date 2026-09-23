/*
 * Copyright (c) Microsoft Corporation.
 * Licensed under the MIT license.
 */

mod search_utils;
#[cfg(test)]
pub use search_utils::{assert_range_results_exactly_match, is_match};
pub use search_utils::{assert_top_k_exactly_match, groundtruth};

/// Resolve the repository `test_data` directory from this crate's own
/// manifest location.
///
/// The `diskann-utils` helper of the same name derives the path from its own
/// manifest, which points into the registry cache once the sibling crates are
/// consumed from crates.io instead of the workspace; these tests need the
/// repository assets, so they resolve the path relative to this crate.
#[cfg(test)]
pub(crate) fn test_data_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("providers crate always resides inside a directory")
        .join("test_data")
}
