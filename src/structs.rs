use std::error::Error;

use futures::future::{lazy, Lazy};

use super::Result;

#[derive(Debug)]
pub struct Choice {
    pub value: usize,
    pub lable: String,
}

impl Choice {
    pub(crate) fn new() -> Self {
        Self {
            value: 0,
            lable: String::new(),
        }
    }

    pub fn parse(v: &serde_json::Value) -> Result<Choice> {
        let lable: String = v
            .get("label")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("family: error label".to_string()))?;

        let value: usize = v
            .get("value")
            .map(|v| v.as_i64().map(|v| v as usize))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("value not readeble".to_string()))?;

        Ok(Choice { lable, value })
    }
}

#[derive(Debug)]
pub struct Response<T> {
    pub count: usize,
    pub result: Vec<T>,
}

impl<T> Response<T> {
    pub fn new(count: usize) -> Response<T> {
        Self {
            count,
            result: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.result.push(value)
    }
}

#[derive(Debug)]
pub struct IPAddress {
    pub id: usize,
    pub address: std::net::IpAddr,
    pub family: Choice,
    pub vrf: Option<Link>,
    pub tenant: Option<Link>,
    // pub role:
    pub interface: Option<IPAddressInterface>,
    // pub nat_inside
    // pub nat_outside
    pub dns_name: String,
    pub description: String,
    pub tags: Vec<String>,
    // custom_fields
    pub created: String, // change
    pub updated: String, // change
}

impl IPAddress {
    pub fn parse(Value: &serde_json::Value) -> Result<IPAddress> {
        let v: &serde_json::Value = Value;
        let id: usize = v
            .get("id")
            .map(|v| v.as_i64())
            .ok_or_else(|| Box::<dyn Error>::from("id invalid".to_string()))?
            .ok_or_else(|| Box::<dyn Error>::from("id invalid".to_string()))?
            as usize;

        let family: Choice = Choice::parse(
            v.get("family")
                .ok_or_else(|| Box::<dyn Error>::from("family missing".to_string()))?,
        )?;

        println!("val: {:?}", family);

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct IPAddressInterface {
    pub id: usize,
    pub url: String,
    pub device: Option<Link>,
    pub virtual_machine: Option<Link>,
    pub name: String,
}

#[derive(Debug)]
pub struct Link {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub extra: (String, Option<String>),
}
