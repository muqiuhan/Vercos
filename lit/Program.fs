module lit.Main

open Argu
open lit.Cli

[<EntryPoint>]
let main argv =
  let result = Parser.Parse(argv)

  match result.GetAllResults() with
  | _ -> printfn $"{Parser.Usage()}"

  0
