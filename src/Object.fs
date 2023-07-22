module Vercos.Object

open Repository

type Object(__repo: Repository, __data: option<string>) as self =
    do
        if __data.IsSome then
            self.Deserialize(__data.Value)

    abstract member Deserialize: string -> unit

    /// This function must be implemented by sub classes
    /// It must read the object's contents from __datra, a byte string.
    /// and do whatever it takes to convert it into a meaningful representation.
    /// What exactly that means depend on each sub class.
    abstract member Serialize: unit -> string