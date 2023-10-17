module Lit.Main

[<EntryPoint>]
let main argv =
  let result = Cli.Parser.Parse(argv)

  match result.GetAllResults() with
  | _ -> printfn $"{Cli.Parser.Usage()}"

  0
