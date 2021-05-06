// Copyright 2017 CoreOS, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Providers
//!
//! These are the providers which Afterburn knows how to retrieve metadata
//! from. Internally, they handle the ins and outs of each providers metadata
//! services, and externally, they provide a function to fetch that metadata in
//! a regular format.
//!
//! To add a provider, put a `pub mod provider;` line in this file, export a
//! function to fetch the metadata, and then add a match line in the top-level
//! `fetch_metadata()` function in metadata.rs.

pub mod aliyun;
pub mod aws;
// pub mod cloudstack;
// pub mod digitalocean;
// pub mod exoscale;
//pub mod gcp;
// pub mod ibmcloud;
// pub mod ibmcloud_classic;
//pub mod microsoft;
pub mod openstack;
// pub mod packet;
// pub mod vmware;
// pub mod vultr;

// use crate::network;
use anyhow::{anyhow, Context, Result};
// use libsystemd::logging;
// use openssh_keys::PublicKey;
use slog_scope::warn;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use users::{self, User};

pub trait MetadataProvider {
    fn attributes(&self) -> Result<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    fn hostname(&self) -> Result<Option<String>> {
        Ok(None)
    }
}
