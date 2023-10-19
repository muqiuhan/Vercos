module Lit.test

open NUnit.Framework
open Repository
open System

[<TestFixture>]
type TestRepositoryPathSolver () =

  [<Test>]
  static member public ``repo_path(".lit", [|"objs" ; "08"|])`` () =
    let repo = Repository(".", false)

    Assert.AreEqual(
      "./.lit/objs/08",
      RepositoryPathSolver(repo).repo_path ([| "objs"; "08" |])
    )

  [<Test>]
  static member public ``If path exists then repo_dir(".lit", [| "refs"; "remotes"; "origin" |])``
    ()
    =
    let path = IO.Path.Join([| ".lit"; "refs"; "remotes"; "origin" |])
    let repo = Repository(".", false)
    IO.Directory.CreateDirectory(path) |> ignore

    Assert.AreEqual(
      "./.lit/refs/remotes/origin",
      RepositoryPathSolver(repo).repo_dir ([| "refs"; "remotes"; "origin" |], false)
    )

    IO.Directory.Delete(path) |> ignore

  [<Test>]
  static member public ``If path not exists then repo_dir(".lit", [| "refs"; "remotes"; "origin"; "HEAD" |], true)``
    ()
    =
    let repo = Repository(".", false)

    let path =
      RepositoryPathSolver(repo)
        .repo_file ([| "refs"; "remotes"; "origin"; "HEAD" |], true)

    Assert.AreEqual("./.lit/refs/remotes/origin", path)
    Assert.AreEqual(true, IO.Directory.Exists(path))

    IO.Directory.Delete(path) |> ignore