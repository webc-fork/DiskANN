/*
 * Copyright (c) Microsoft Corporation.
 * Licensed under the MIT license.
 */

use diskann_utils::{strided, views::DenseData};
use webc_diskann::ANNError;

use crate::utils::Bridge;

// crates.io `diskann-utils` 0.59.0 models `strided::TryFromError` as a generic
// struct carrying the borrowed data; flatten it to the `'static` light variant
// so it can enter an `ANNError`.
impl<T: DenseData> From<Bridge<strided::TryFromError<T>>> for ANNError {
    #[track_caller]
    fn from(value: Bridge<strided::TryFromError<T>>) -> Self {
        ANNError::new(value.into_inner().as_static())
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::BridgeErr;

    #[test]
    fn test_conversion() {
        let nrows = 5;
        let ncols = 3;

        let x = vec![u8::default(); nrows * ncols];

        // Provided the incorrect dimensions.
        let err = strided::StridedView::try_shrink_from(&x, nrows, ncols + 1, ncols + 1)
            .bridge_err()
            .unwrap_err();
        let message = format!("{}", err);

        let ann = ANNError::from(err);
        let formatted = ann.to_string();
        assert!(formatted.contains(&message));
    }
}
