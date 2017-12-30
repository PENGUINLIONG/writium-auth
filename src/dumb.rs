use writium::prelude::*;
use auth::Authority;

const ERR_DUMB: &'static str = "An dumb authority is used, any privilege is not granted.";

pub struct DumbAuthority();
impl DumbAuthority {
    pub fn new() -> DumbAuthority {
        DumbAuthority()
    }
}
impl Authority for DumbAuthority {
    type Privilege = ();
    fn authorize(&self, _pr: Self::Privilege, _req: &Request) -> Result<()> {
        Err(Error::new(StatusCode::Unauthorized, ERR_DUMB))
    }
}
