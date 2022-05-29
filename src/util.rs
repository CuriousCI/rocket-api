use crate::*;

pub fn to_uuid(id: Guid) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(id.to_string().as_str())
}
