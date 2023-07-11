module Vercos.ConfigFile

open IniParser

type ConfigFile(__path: string) =
    let __parser = FileIniDataParser()
    member public this.Data = __parser.ReadFile(__path)
    member public this.Save () = __parser.WriteFile(__path, this.Data)