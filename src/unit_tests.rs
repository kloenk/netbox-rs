#[test]
fn check_succes() -> super::Result<()> {
    super::NetBoxBuilder::new(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "http://netbox.example.com/api/",
    )
    .map(|_| ())
}

#[test]
fn check_token() {
    super::NetBoxBuilder::new("e", "http://netbox.example.com/api/").unwrap_err();
}

#[test]
fn check_url() {
    super::NetBoxBuilder::new(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "http://netbox.example.com",
    )
    .unwrap_err();
}

#[test]
fn build() -> super::Result<()> {
    super::NetBoxBuilder::new(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "http://netbox.example.com/api/",
    )?
    .build()
    .map(|_| ())
}
