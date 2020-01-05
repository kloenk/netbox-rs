use std::error::Error;
use std::collections::HashMap;

use reqwest::{Client, Response};

use semver::Version;



#[macro_use]
use log::{trace, debug, info, warn};

#[cfg(test)]
mod unit_tests;

pub mod builder;
pub use builder::NetBoxBuilder;

pub mod structs;

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

      self.get(&self.base_url.clone()).await?;
      if let Some(ver) = &self.version_cache {
        return Ok(ver)
      } else {
          unimplemented!();
      }
    }

    pub async fn get_root(&mut self, path: &str) -> Result<HashMap<String, String>> {
      let url = self.base_url.clone();
      let url = format!("{}{}", url, path);
      let body = self.get(&url).await?;
      let mut ret = HashMap::new();

      let body = body.text().await?;

      use serde_json::Value;
      let v: Value = serde_json::from_str(&body)?;

      if let Some(v) = v.as_object() {
        for (k, v) in v.iter() {
            if let Some(v) = v.as_str() {
              ret.insert(k.clone(), v.to_string());
            }
        }
      }

      Ok(ret)
    }
    
    pub async fn get_choices(&mut self, path: &str) -> Result<HashMap<String, Vec<structs::Choice>>> {
      let url = self.base_url.clone();
      let url = format!("{}{}/_choices", url, path);
      let body = self.get(&url).await?;
      let mut ret = HashMap::new();

      let body = body.text().await?;

      use serde_json::Value;
      let v: Value = serde_json::from_str(&body)?;

      if let Some(v) = v.as_object() {
        for (k, v) in v.iter() {
          let mut choices = Vec::new();
          if let Some(v) = v.as_array() {
            for v in v.iter() {
              let mut choice = structs::Choice::new();
              if let Some(v) = v.get("value") {
                if let Some(v) = v.as_i64() {
                  choice.value = v as usize;
                }
              }
              if let Some(v) = v.get("label") {
                if let Some(v) = v.as_str() {
                  choice.lable = v.to_string();
                }
              }
              choices.push(choice);
            }
          }
          ret.insert(k.to_string(), choices);
        }
      }

      Ok(ret)
    }

    pub async fn get(&mut self, url: &str) -> Result<Response> {
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

      Ok(req)
    }
    
}
