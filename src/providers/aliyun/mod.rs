//! Alibaba Cloud
//!
//! This provider is selected via the platform ID `aliyun`.
//! The metadata endpoint is documented at https://www.alibabacloud.com/help/doc-detail/49122.htm.

use anyhow::{anyhow, Result};
#[cfg(test)]
use mockito;

use slog_scope::error;
use std::collections::{BTreeSet, HashMap};

use crate::providers::MetadataProvider;
use crate::retry;

#[cfg(test)]
mod mock_tests;

/// Provider prefix for Alibaba Cloud.
static PROVIDER_PREFIX: &str = "ALIYUN";

#[derive(Clone, Debug)]
pub struct AliyunProvider {
    client: retry::Client,
}

impl AliyunProvider {
    pub fn try_new() -> Result<AliyunProvider> {
        let client = retry::Client::try_new()?.return_on_404(true);

        Ok(AliyunProvider { client })
    }

    #[cfg(test)]
    fn endpoint_for(name: &str) -> String {
        let url = mockito::server_url();
        format!("{}/{}", url, name)
    }

    #[cfg(not(test))]
    fn endpoint_for(name: &str) -> String {
        format!("http://100.100.100.200/latest/meta-data/{}", name)
    }

    /// Fetch a metadata attribute from its specific endpoint.
    ///
    /// Content (if any) is stored into the provided `map`,
    /// overwriting any previous existing value under the same `key`.
    fn fetch_attribute(
        &self,
        map: &mut HashMap<String, String>,
        key: &str,
        endpoint: &str,
    ) -> Result<()> {
        let content: Option<String> = self
            .client
            .get(retry::Raw, Self::endpoint_for(endpoint))
            .send()?;

        if let Some(value) = content {
            if !value.is_empty() {
                map.insert(key.to_string(), value);
            }
        }

        Ok(())
    }

    /// Retrieve hostname.
    fn fetch_hostname(&self) -> Result<Option<String>> {
        let value: Option<String> = self
            .client
            .get(retry::Raw, AliyunProvider::endpoint_for("hostname"))
            .send()?;

        let hostname = value.unwrap_or_default();
        if hostname.is_empty() {
            return Ok(None);
        }

        Ok(Some(hostname))
    }
}

impl MetadataProvider for AliyunProvider {
    fn attributes(&self) -> Result<HashMap<String, String>> {
        // See https://www.alibabacloud.com/help/doc-detail/49122.htm.
        let mut out = HashMap::with_capacity(10);

        self.fetch_attribute(&mut out, &format!("{}_EIPV4", PROVIDER_PREFIX), "eipv4")?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_HOSTNAME", PROVIDER_PREFIX),
            "hostname",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_IMAGE_ID", PROVIDER_PREFIX),
            "image-id",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_INSTANCE_ID", PROVIDER_PREFIX),
            "instance-id",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_INSTANCE_TYPE", PROVIDER_PREFIX),
            "instance/instance-type",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_IPV4_PRIVATE", PROVIDER_PREFIX),
            "private-ipv4",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_IPV4_PUBLIC", PROVIDER_PREFIX),
            "public-ipv4",
        )?;
        self.fetch_attribute(
            &mut out,
            &format!("{}_REGION_ID", PROVIDER_PREFIX),
            "region-id",
        )?;
        self.fetch_attribute(&mut out, &format!("{}_VPC_ID", PROVIDER_PREFIX), "vpc-id")?;
        self.fetch_attribute(&mut out, &format!("{}_ZONE_ID", PROVIDER_PREFIX), "zone-id")?;

        Ok(out)
    }

    fn hostname(&self) -> Result<Option<String>> {
        self.fetch_hostname()
    }
}
