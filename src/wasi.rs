/// Raw display handle for the Wasi.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WasiDisplayHandle {
    pub handle: u32,
}

impl WasiDisplayHandle {
    /// Create a new handle to a graphics-context.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WasiDisplayHandle;
    ///
    /// let handle: u32 = 0; // context.handle();
    /// let display_handle = WasiDisplayHandle::new(handle);
    /// ```
    pub fn new(handle: u32) -> Self {
        Self { handle }
    }
}

/// Raw window handle for the Wasi.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WasiWindowHandle {
    /// A handle to a graphics-context resource.
    pub handle: u32,
}

impl WasiWindowHandle {
    /// Create a new handle to a graphics-context.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WasiWindowHandle;
    /// #
    /// let handle: u32 = 0; // context.handle();
    /// let window_handle = WasiWindowHandle::new(handle);
    /// ```
    pub fn new(handle: u32) -> Self {
        Self { handle }
    }
}
