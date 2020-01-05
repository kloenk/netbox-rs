use std::error::Error;

use reqwest::Client;

use semver::Version;

#[macro_use]
use log::{trace, debug, info, warn};

#[cfg(test)]
mod unit_tests;

pub mod builder;
pub use builder::NetBoxBuilder;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct NetBox {
    // the base url of nebox (including /api/)
    base_url: String,

    //cache
    version_cache: Option<Version>,

    // client
    client: Client,
}

impl NetBox {
    pub fn new(token: &str, base_url: &str) -> Result<Self> {
      NetBoxBuilder::new(token, base_url)?.build()
    }

    // requst functions
    pub async fn version(&mut self) -> Result<&Version> {
      if self.version_cache.is_some() {
        trace!("use cached version");
        return Ok(self.version_cache.as_ref().unwrap());
      }

      self.get_root().await?;
      if let Some(ver) = &self.version_cache {
        return Ok(ver)
      } else {
          unimplemented!();
      }
    }

    pub async fn get_root(&mut self) -> Result<()> {
      let url = self.base_url.clone();
      let body = self.get(&url).await?;



      Ok(())
    }

    pub async fn get(&mut self, url: &str) -> Result<()> {
      let req = self.client.get(url);
      let req = req.send().await?;
      let headers = req.headers();

      if let Some(api) = headers.get("API-Version") {
        let mut ver = api.to_str()?.to_string();
        if ver.len() == 3 {
          ver = format!("{}.0", ver);
        }
        let ver = Version::parse(&ver)?;
        
        self.version_cache = Some(ver);
      }

      Ok(())
    }
    
}
