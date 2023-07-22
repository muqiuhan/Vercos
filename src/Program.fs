module Vercos.Main

open Cli

[<EntryPoint>]
let main args =
    Cli.parse (args)
    0
