module Lit.Main

let init () =
  Repository.init (".") |> fun repo -> printfn $"{repo}"

[<EntryPoint>]
let main argv =
  let result = Cli.Parser.parse (argv)

  match result.GetAllResults() with
  | Cli.Argu_impl.cli_arguments.Init(_) :: _ -> init ()
  | _ -> printfn $"{Cli.Parser.usage ()}"

  0
