module Lit.Repository

open System
open IniParser
open Errors

type Repository =
  { worktree : string
    gitdir : string
    conf : Model.IniData }


/// compute some paths and create missing directory structures if needed.
module Path =
  /// Compute path under repo's gitdir.
  let repo_path (gitdir : string, path : array<string>) =
    let path = Array.append [| gitdir |] path
    IO.Path.Join(path)

  /// Same as repo_path, but mkdir *path if absent if mkdir.
  let repo_dir (gitdir : string, path : array<string>, mkdir : bool) =
    let path = repo_path (gitdir, path)

    if IO.Path.Exists(path) then
      if IO.Directory.Exists(path) then
        Ok(path)
      else
        Error(Repository(NotDirectory(path)))
    else if mkdir then
      Log.Debug $"Create directory {path}"
      IO.Directory.CreateDirectory(path) |> ignore
      Ok(path)
    else
      Error(Unknown)

  /// Same as repo_path, but create dirname(*path) if absent.
  /// For example, repo_file(r, \"refs\", \"remotes\", \"origin\", \"HEAD\") will create
  /// .lit/refs/remotes/origin.
  let repo_file (gitdir : string, path : array<string>, mkdir : bool) =
    let dir_path = Array.sub path 1 (Array.length path - 1)

    repo_dir (gitdir, dir_path, mkdir)
    |> Result.map (fun dir_path -> repo_path (gitdir, path))

/// Get repository config file
let GetConfig (gitdir : string) =
  let conf = new FileIniDataParser()

  Result.map
    (fun path ->
      if IO.Path.Exists(path) then
        conf.ReadFile(path)
      else
        Log.Fatal(Repository(ConfigFileMissing(path)).ToString())
        failwith "")
    (Path.repo_file (gitdir, [| "config" |], false))

/// Check repositoryformatversion
let CheckRepositoryFormatVersion (conf : Model.IniData) =
  let version = conf["core"]["repositoryformatversion"]

  if version <> "0" then
    Error(UnsupportedRepositoryFormatVersion(version))
  else
    Ok(())

/// Optional [force] disables all checks.
let Make (path : string, force : bool) =
  let worktree = path
  let gitdir = IO.Path.Join(path, ".git")

  if not (force || IO.Directory.Exists(gitdir)) then
    Error(Repository(NotLitRepository path))
  else if not force then
    Result.map
      (fun conf ->
        Result.map
          (fun () ->
            { gitdir = gitdir
              worktree = worktree
              conf = conf })
          (CheckRepositoryFormatVersion(conf)))
      (GetConfig(gitdir))
  else
    Error(Unknown)
