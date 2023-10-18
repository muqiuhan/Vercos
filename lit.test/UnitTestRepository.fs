module Lit.test

open NUnit.Framework
open Repository
open System

[<TestFixture>]
module Path =
  open Path

  [<Test>]
  let ``repo_path(".lit", [|"objs" ; "08"|])`` () =
    Assert.AreEqual(".lit/objs/08", repo_path (".lit", [| "objs"; "08" |]))

  [<Test>]
  let ``If path exists then repo_dir(".lit", [| "refs"; "remotes"; "origin" |])`` () =
    let path = IO.Path.Join([| ".lit"; "refs"; "remotes"; "origin" |])
    IO.Directory.CreateDirectory(path) |> ignore

    Assert.AreEqual(
      true,
      Result.map
        (fun path -> Assert.AreEqual(".lit/refs/remotes/origin", path))
        (repo_dir (".lit", [| "refs"; "remotes"; "origin" |], false))
      |> Result.mapError (fun err -> failwith $"{err}")
      |> Result.isOk
    )


    IO.Directory.Delete(path) |> ignore

  [<Test>]
  let ``If path not exists then repo_dir(".lit", [| "refs"; "remotes"; "origin" |], true)``
    ()
    =
    let path = IO.Path.Join([| ".lit"; "refs"; "remotes"; "origin" |])

    Assert.AreEqual(
      true,
      Result.map
        (fun path -> Assert.AreEqual(".lit/refs/remotes/origin", path))
        (repo_dir (".lit", [| "refs"; "remotes"; "origin" |], true))
      |> Result.mapError (fun err -> failwith $"{err}")
      |> Result.isOk
    )

    Assert.AreEqual(true, IO.Directory.Exists(path))
    IO.Directory.Delete(path) |> ignore
