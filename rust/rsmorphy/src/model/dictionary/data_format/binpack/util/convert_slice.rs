use std::mem;
use std::slice;

use thiserror::Error;

/// Converts slice of one type to slice of another type.
///
/// # Panics
///
/// If output slice is not aligned.
/// It can be fixed by adding `#[repr(align(N))]` to the input container, where `N` is the
/// correct alignment, depending on the platform.
///
/// This panic will be replaced with respective error when corresponding features will be
/// stabilized. See [https://github.com/rust-lang/rust/issues/96284],
/// [https://github.com/rust-lang/rust/issues/104203]
///
/// # Safety
///
/// This function checks that the size of the input slice matches the size of the output slice,
/// and the elements of one slice do not cross the boundaries of the elements of another.
///
/// BUT!
///
/// The calling side is guaranteed that the elements of the input slice can be safely
/// transmuted to the elements of the output slice.
#[inline]
pub const unsafe fn convert_slice<In, Out>(input: &[In]) -> Result<&[Out], ConvertSliceError> {
    let in_size = mem::size_of::<In>();
    let out_size = mem::size_of::<Out>();

    let input_len = input.len();
    let input_size = input_len * in_size;

    if in_size != out_size {
        // The size of output slice shouldn't differ from the size of input slice.
        if input_size % out_size != 0 {
            return Err(ConvertSliceError::mismatched_size(input_size, out_size));
        }
        let (mx, mn) = (max(in_size, out_size), min(in_size, out_size));
        if mx % mn != 0 {
            return Err(ConvertSliceError::crossed_bounds(in_size, out_size));
        }
    }

    let out_ptr = input.as_ptr() as *const Out;

    // FIXME uncomment when `pointer_is_aligned` and `const_pointer_is_aligned` will be stabilized.
    // if !out_ptr.is_aligned() {
    //     return Err(SliceConvertError::InvalidAlignment);
    // }

    let output_len = input_size / out_size;
    Ok(slice::from_raw_parts(out_ptr, output_len))
}

// FIXME remove when std::cmp::max will be const.
#[inline(always)]
const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

// FIXME remove when std::cmp::min will be const.
#[inline(always)]
const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Debug, Clone, Copy, Error)]
pub enum ConvertSliceError {
    #[error("Mismatched size: {input_size} is not a multiple of {out_size}")]
    MismatchedSize { input_size: usize, out_size: usize },
    #[error("Crossed bounds: elements of {in_size} crossed bounds of {out_size}")]
    CrossedBounds { in_size: usize, out_size: usize },
    // FIXME uncomment when `pointer_is_aligned` and `const_pointer_is_aligned` will be stabilized
    // #[error("Invalid alignment of output slice")]
    // InvalidAlignment,
}

impl ConvertSliceError {
    #[inline(always)]
    const fn mismatched_size(input_size: usize, out_size: usize) -> Self {
        ConvertSliceError::MismatchedSize {
            input_size,
            out_size,
        }
    }

    #[inline(always)]
    const fn crossed_bounds(in_size: usize, out_size: usize) -> Self {
        ConvertSliceError::CrossedBounds { in_size, out_size }
    }
}
