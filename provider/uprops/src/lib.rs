// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_uprops` contains implementations of the [`ICU4X`]
//! [`DataProvider`] interface backed by TOML files exported by the
//! ICU4C icuwriteuprops tool. Create a directory containing TOML files for
//! the necessary Unicode properties and then pass the path into the
//! [`PropertiesDataProvider`].
//!
//! **Important:** This data provider implementation is not optimized
//! for production use.  It is much more efficient if you use
//! [`FsDataProvider`] or [`StaticDataProvider`] instead.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: icu_provider::prelude::DataProvider
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html
//! [`PropertiesDataProvider`]: binary::PropertiesDataProvider

mod binary;
mod enumerated;
mod error;
mod provider;
mod uprops_serde;

pub use provider::PropertiesDataProvider;
