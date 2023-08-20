use std::cmp::Ordering;

use bytes::Bytes;

use trust_dns_resolver::config::ResolverConfig;

#[cfg(feature = "runtime-tokio")]
use trust_dns_resolver::TokioAsyncResolver;

#[cfg(feature = "runtime-async-std")]
use async_std_resolver::{resolver, AsyncStdResolver};

use trust_dns_proto::rr::rdata::MX;

use crate::error::Result;

pub struct Dns {
    #[cfg(feature = "runtime-tokio")]
    resolver: TokioAsyncResolver,
    #[cfg(feature = "runtime-async-std")]
    resolver: AsyncStdResolver,
}

pub struct SortableMX {
    mx: MX,
}

impl Ord for SortableMX {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mx.preference().cmp(&other.mx.preference())
    }
}

impl PartialOrd for SortableMX {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SortableMX {}

impl PartialEq for SortableMX {
    fn eq(&self, other: &Self) -> bool {
        self.mx.preference() == other.mx.preference()
    }
}

impl Dns {
    pub async fn new() -> Result<Self> {
        #[cfg(feature = "runtime-tokio")]
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), Default::default())?;

        #[cfg(feature = "runtime-async-std")]
        let resolver = resolver(ResolverConfig::default(), Default::default()).await?;

        let dns = Self { resolver };

        Ok(dns)
    }

    pub async fn get_txt<N: AsRef<str>>(&self, name: N) -> Result<Vec<Bytes>> {
        let lookup_results = self.resolver.txt_lookup(name.as_ref()).await?;

        let mut records: Vec<_> = Vec::new();

        for txt in lookup_results {
            let mut bytes: Vec<Bytes> = txt
                .txt_data()
                .iter()
                .map(|data| data.to_vec().into())
                .collect();

            if bytes.first().is_some() {
                let record = bytes.remove(0);

                records.push(record);
            }
        }

        Ok(records)
    }
}
