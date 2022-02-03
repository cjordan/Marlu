use crate::context::MarluVisContext;

mod error;
pub mod ms;

cfg_if::cfg_if! {
    if #[cfg(feature = "mwalib")] {
        use std::ops::Range;

        use crate::{mwalib::CorrelatorContext, Jones};
        use ndarray::{ArrayView3, ArrayView4, ArrayViewMut3};
        use self::error::IOError;
    }
}

/// The container has visibilities which can be read by passing in a mwalib
/// context and the range of values to read.
pub trait VisReadable: Sync + Send {
    /// Read the visibilities and weights for the selected timesteps, coarse
    /// channels and baselines into the provided arrays.
    ///
    /// # Errors
    ///
    /// Can throw IOError if there is an issue reading.
    ///
    /// TODO: reduce number of arguments.
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "mwalib")]
    fn read_vis_mwalib(
        &self,
        jones_array: ArrayViewMut3<Jones<f32>>,
        weight_array: ArrayViewMut3<f32>,
        context: &CorrelatorContext,
        timestep_range: &Range<usize>,
        coarse_chan_range: &Range<usize>,
        baseline_idxs: &[usize],
    ) -> Result<(), IOError>;
}

/// The container can accept a chunk of visibilities to be written.
pub trait VisWritable: Sync + Send {
    /// Specify the chunk of visibilities to write using a [`MarluVisContext`].
    fn write_vis_marlu(
        &mut self,
        vis: ArrayView3<Jones<f32>>,
        weights: ArrayView3<f32>,
        flags: ArrayView3<bool>,
        context: &MarluVisContext,
        draw_progress: bool,
    ) -> Result<(), IOError>;

    /// Write visibilities and weights from the arrays. Timestep, coarse channel
    /// and baseline indices are needed for labelling the visibility array
    ///
    /// `jones_array` - a three dimensional array of jones matrix visibilities.
    ///     The dimensions of the array are `[timestep][channel][baseline]`
    ///
    /// `weight_array` - a four dimensional array of visibility weights.
    ///     The dimensions of the array are `[timestep][channel][baseline][pol]`
    ///
    /// `flag_array` - a four dimensional array of visibility flags.
    ///     The dimensions of the array are `[timestep][channel][baseline][pol]`
    ///
    /// `context` - a [`mwalib::CorrelatorContext`] object containing information
    ///     about the timesteps, channels and baselines referred to in the jones
    ///     array.
    ///
    /// `timestep_range` - the range of indices into `CorrelatorContext.timesteps`
    ///     corresponding to the first dimension of the jones array.
    ///
    /// `coarse_chan_range` - the range of indices into `CorrelatorContext.coarse_chans`
    ///     corresponding to the second dimension of the jones array.
    ///     Note: this is a range of coarse channels, where the jones array is a range
    ///     of fine channels
    ///
    /// `baseline_idxs` - the range of indices into `CorrelatorContext.metafits_context.baselines`
    ///     corresponding to the third dimension of the jones array.
    ///
    /// `avg_time` - the number of timesteps to average together.
    ///
    /// `avg_freq` - the number of channels to average together.
    ///
    /// `draw_progress` - whether or not to draw a progress bar.
    ///
    /// # Errors
    ///
    /// Can throw IOError if there is an issue writing to the file, or the indices
    /// into `context` are invalid.
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "mwalib")]
    fn write_vis_mwalib(
        &mut self,
        jones_array: ArrayView3<Jones<f32>>,
        weight_array: ArrayView4<f32>,
        flag_array: ArrayView4<bool>,
        context: &CorrelatorContext,
        timestep_range: &Range<usize>,
        coarse_chan_range: &Range<usize>,
        baseline_idxs: &[usize],
        avg_time: usize,
        avg_freq: usize,
        draw_progress: bool,
    ) -> Result<(), IOError>;
}
