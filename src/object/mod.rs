mod reading;
mod writing;

trait Object {
    fn to_string(&self) -> String;
    fn deserialize(data: &String) -> Self
    where
        Self: Sized;

    /// It must read the object's contents from data, a byte string, and do
    /// whatever it takes to convert it into a meaningful representation.
    fn serialize(&self) -> &[u8];

    fn fmt(&self) -> &String;
}

struct Blob {
    fmt: String,
}

impl Object for Blob {
    fn to_string(&self) -> String {
        todo!()
    }

    fn deserialize(data: &String) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn serialize(&self) -> &[u8] {
        todo!()
    }

    fn fmt(&self) -> &String {
        &self.fmt
    }
}
