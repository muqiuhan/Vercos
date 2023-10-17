module vercos.Main

open Argu
open vercos.Argu

[<EntryPoint>]
let main argv =
  let result = Parser.Parse(argv)

  match result.GetAllResults() with
  | _ -> printfn $"{Parser.Usage()}"

  0
