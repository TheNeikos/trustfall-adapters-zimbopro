mod adapter_impl;
mod edges;
mod entrypoints;
mod properties;
pub mod vertex;

#[cfg(test)]
mod tests;

pub use adapter_impl::FileSystemAdapter;
