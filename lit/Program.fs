module Lit.Main

let init () =
  ()

[<EntryPoint>]
let main argv =
  let result = Cli.Parser.parse (argv)

  match result.GetAllResults() with
  | Cli.Argu_impl.cli_arguments.Init(_) :: _ -> init ()
  | _ -> printfn $"{Cli.Parser.usage ()}"

  0
