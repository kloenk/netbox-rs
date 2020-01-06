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
    pub mask: u8,
    pub family: Choice,
    pub vrf: Option<Link>,
    pub tenant: Option<Link>,
    pub status: Choice,
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
    pub fn parse(value: &serde_json::Value) -> Result<IPAddress> {
        let v: &serde_json::Value = value;
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

        let (address, mask) = v
            .get("address")
            .map(|v| v.as_str())
            .flatten()
            .map(|v| v.split('/').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .ok_or_else(|| Box::<dyn Error>::from("could not parse address".to_string()))?;
        let address: Result<std::net::IpAddr> = {
            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
            if address.contains(':') {
                let address = address.replace("::", ":fill:");
                let address = address.split(':').collect::<Vec<&str>>();
                let mut buff = Vec::new();
                let mut c = 9 - address.len();

                for v in address {
                    if v == "fill" {
                        for _ in 0..c {
                            buff.push(0);
                        }
                    } else {
                        let v = i32::from_str_radix(v, 16)? as u16;
                        buff.push(v)
                    }
                }
                Ok(IpAddr::V6(Ipv6Addr::new(
                    buff[0], buff[1], buff[2], buff[3], buff[4], buff[5], buff[6], buff[7],
                )))
            } else {
                let v = address.split(".").collect::<Vec<&str>>();
                if v.len() != 4 {
                    return Err(Box::<dyn Error>::from(
                        "could not parse address".to_string(),
                    ));
                }
                Ok(IpAddr::V4(Ipv4Addr::new(
                    v[0].parse()?,
                    v[1].parse()?,
                    v[2].parse()?,
                    v[3].parse()?,
                )))
            }
        };
        let address = address?;
        let mask: u8 = mask.parse()?;

        let vrf: Option<Link> = v.get("vrf").map(|v| Link::parse(v).ok()).flatten();
        let tenant: Option<Link> = v.get("tenant").map(|v| Link::parse(v).ok()).flatten();
        let status: Choice = Choice::parse(
            v.get("status")
                .ok_or_else(|| Box::<dyn Error>::from("status missing".to_string()))?,
        )?;

        // FIMXE: role

        let interface: Option<IPAddressInterface> = v
            .get("interface")
            .map(|v| IPAddressInterface::parse(v).ok())
            .flatten();

        // FIXME: nat
        let dns_name: String = v
            .get("dns_name")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .unwrap_or_else(|| String::new());

        let description: String = v
            .get("description")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .unwrap_or_else(|| String::new());

        let tags: Vec<String> = Vec::new(); // TODO
        let created: String = String::new(); // TODO
        let updated: String = String::new(); // TODO

        Ok(Self {
            id,
            family,
            address,
            mask,
            vrf,
            tenant,
            status,
            //role
            interface,
            //nat
            //nat
            dns_name,
            description,
            tags,
            //custom
            created,
            updated,
        })
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

impl IPAddressInterface {
    pub fn parse(value: &serde_json::Value) -> Result<Self> {
        let id: usize = value
            .get("id")
            .map(|v| v.as_i64().map(|v| v as usize))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse id".to_string()))?;
        let url: String = value
            .get("url")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse url".to_string()))?;
        let name: String = value
            .get("name")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse name".to_string()))?;
        let device: Option<Link> = value.get("device").map(|v| Link::parse(v).ok()).flatten();
        let virtual_machine: Option<Link> = value
            .get("virtual_machine")
            .map(|v| Link::parse(v).ok())
            .flatten();
        Ok(Self {
            id,
            url,
            name,
            device,
            virtual_machine,
        })
    }
}

#[derive(Debug)]
pub struct Link {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub extra: (String, Option<String>),
}

impl Link {
    pub fn parse(value: &serde_json::Value) -> Result<Self> {
        let id: usize = value
            .get("id")
            .map(|v| v.as_i64().map(|v| v as usize))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse id".to_string()))?;
        let url: String = value
            .get("url")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse url".to_string()))?;
        let name: String = value
            .get("name")
            .map(|v| v.as_str().map(|v| v.to_string()))
            .flatten()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse name".to_string()))?;
        let value = value
            .as_object()
            .ok_or_else(|| Box::<dyn Error>::from("could not parse extra".to_string()))?;

        let mut iter = value
            .iter()
            .filter(|x| x.0 != "id" && x.0 != "url" && x.0 != "name");

        if let Some((k, v)) = iter.next() {
            let v = v.as_str().map(|v| v.to_string());
            return Ok(Self {
                id,
                url,
                name,
                extra: (k.to_string(), v),
            });
        }

        return Ok(Self {
            id,
            url,
            name,
            extra: (String::new(), None),
        });
    }
}
