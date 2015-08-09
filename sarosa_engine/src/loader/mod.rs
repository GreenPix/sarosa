
// Re-export for doc.
pub use core::loops::deferred::DeferredLoader;

/// Trait to define a loader that could be loaded
/// using the `DeferredLoader`.
///
/// TODO(Nemikolh): Add also an interface to do
/// incremental loading with reports after each step.
/// (Can be usefull to display a percentage or what is being
/// currently made).
///
/// Probably Something like:
/// ```ignore
/// struct NextResourceInfo {
///     name: String,
///     progress: f32,
/// }
/// // To add to the trait Loader.
/// fn next_resource_kind() -> NextResourceInfo;
/// fn load_next_resource();
/// ```
pub trait Loader: Send {

    type Resources: Send;

    /// This function is supposed to do
    /// all the loading for R. Here, you should
    /// be doing long operation like reading a file,
    /// and so on.
    fn load_resources(&mut self) -> Self::Resources;
}
