module Lit.Main

[<EntryPoint>]
let main argv =
  let result = Cli.Parser.parse (argv)

  match result.GetAllResults() with
  | _ -> printfn $"{Cli.Parser.usage ()}"

  0
