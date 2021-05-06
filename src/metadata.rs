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

use anyhow::{bail, Result};

use crate::providers;
use crate::providers::aliyun::AliyunProvider;
use crate::providers::aws::AwsProvider;
use crate::providers::openstack;
use crate::providers::openstack::network::OpenstackProviderNetwork;

macro_rules! box_result {
    ($exp:expr) => {
        Ok(Box::new($exp))
    };
}

/// Fetch metadata for the given provider.
///
/// This is the generic, top-level function to fetch provider metadata.
/// The configured provider is passed in and this function dispatches the call
/// to the provider-specific fetch logic.
pub fn fetch_metadata(provider: &str) -> Result<Box<dyn providers::MetadataProvider>> {
    match provider {
        "aliyun" => box_result!(AliyunProvider::try_new()?),
        "aws" => box_result!(AwsProvider::try_new()?),
        "huaweicloud" => box_result!(OpenstackProviderNetwork::try_new()?),
        "openstack" => box_result!(OpenstackProviderNetwork::try_new()?),
        _ => bail!("unknown provider '{}'", provider),
    }
}
