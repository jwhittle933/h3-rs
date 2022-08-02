pub type H3Error = u32;

pub enum H3ErrorCode {
    /// no error
    Success,
    /// the operation failed but a more specific error is not
    /// available.
    Failed,
    /// Argument was outside of acceptable range
    /// (when a more specific error code is not availible).
    Domain,
    /// Latitude or longitude arguments were outside of acceptable range.
    LatLngDomain,
    /// Resolution arguments were outside of acceptable range.
    ResDomain,
    /// [`H3Index`] cell argument was not valid.
    CellInvalid,
    /// [`H3Index`] directed edge argument was not valid.
    DirEdgeInvalid,
    /// [`H3Index`] undirected edge argument was not valid.
    UnDirEdgeInvalid,
    /// Pentagon distortion was encountered which the algorithm could not handle.
    Pentagon,
    /// Duplicate input was encountered in the arguments and the algorithm
    /// could not handle it.
    DuplicateInput,
    /// [`H3Index`] cell arguments were not neighbors.
    NotNeighbors,
    /// [`H3Index`] cell arguments had incompatible resolutions.
    ResMismatch,
    /// Necessary memory allocation failed.
    MemoryAlloc,
    /// Bounds of provided memory were not large enough.
    MemoryBounds,
    /// Mode or flags argument was not valid.
    OptionInvalid,
}
