module Lit.Repository

open System
open IniParser
open Errors

type Repository (path : string, force : bool) as self =
  do self.check_path ()

  member public this.worktree = path
  member public this.litdir = IO.Path.Combine([| path; ".lit" |])
  member public this.conf = new Config(self)

  member private this.check_path () =
    if not (force || IO.Path.Exists(this.litdir)) then
      Error(Errors.Repository(Not_lit_repository(path))) |> unwrap

/// Manipulating lots of paths in repositories
and RepositoryPathSolver (repo : Repository) =

  /// Compute path under repo's litdir
  member public this.repo_path (path : array<string>) =
    IO.Path.Combine(Array.append [| repo.litdir |] path)

  /// Same as repo_path, but create dirname(*path) if absent.
  /// For example, repo_file(r, "refs", "remotes", "origin", "HEAD")
  ///   will create .lit/refs/remotes/origin.
  member public this.repo_file (path : array<string>, mkdir : bool) =
    this.repo_dir (Array.sub path 1 (Array.length path - 1), mkdir) |> unwrap

  /// Same as repo_path, but mkdir path if absent if mkdir.
  member public this.repo_dir (path : array<string>, mkdir : bool) =
    let path = this.repo_path (path)

    if IO.Path.Exists(path) then
      if IO.Directory.Exists(path) then
        Ok(path)
      else
        Error(Errors.Repository(Not_directory(path)))
    else if mkdir then
      IO.Directory.CreateDirectory(path) |> ignore
      Ok(path)
    else
      Error(Errors.Repository(CannotGetDirectory(path)))

and Config (repo : Repository) =
  let parser = new FileIniDataParser()
  let conf = new Model.IniData()

  // do parser.ReadFile(repo.litdir)
