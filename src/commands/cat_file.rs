/// Provide content of repository objects
pub struct CatFile {
    /// Specify the type (blob | commit | tag | tree)
    pub typ: String,

    /// The object to display
    pub object: String,
}

impl CatFile {
    pub fn cat(&self) {}
}
