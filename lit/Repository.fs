module Lit.Repository

open System
open IniParser
open Errors

type repository =
  { worktree : string
    gitdir : string
    conf : string }

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
        Error(Repository(Not_directory(path)))
    else if mkdir then
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
    |> function
      | Ok path -> path
      | err -> failwith $"{err}"

/// Get repository config file
let get_config (gitdir : string) =
  let path = Path.repo_file (gitdir, [| "config" |], false)

  if IO.Path.Exists(path) then
    path
  else
    failwith $"{Repository(Config_file_missing(path))}"

/// Check repositoryformatversion
let check_format_version (conf : string) =
  let parser = new FileIniDataParser()
  let version = (parser.ReadFile(conf)).["core"]["repositoryformatversion"]

  if version <> "0" then
    failwith $"{Repository(Unsupported_repository_format_version(version))}"

/// Optional [force] disables all checks.
let make (path : string, force : bool) =
  let worktree = path
  let gitdir = IO.Path.Join(path, ".git")

  if not (force || IO.Directory.Exists(gitdir)) then
    failwith $"{Repository(Not_lit_repository path)}"
  else if not force then
    let conf = get_config (gitdir)
    check_format_version (conf)

    { gitdir = gitdir
      worktree = worktree
      conf = conf }
  else
    failwith $"{Unknown}"

/// Repository default config
let DEFAULT_CONFIG =
  let conf = new Model.IniData()
  conf["core"]["repositoryformatversion"] <- "0"
  conf["core"]["filemode"] <- "false"
  conf["core"]["bare"] <- "false"
  conf

// Make sure the path either doesn't exist or is an empty dir.
let check_path (path : string) =
  let repo = make (path, true)

  if IO.Path.Exists(repo.worktree) then
    if not (IO.Directory.Exists(repo.worktree)) then
      failwith $"{Repository(Not_directory(repo.worktree))}"
    else if
      (IO.Directory.GetFiles(repo.gitdir).Length <> 0)
      && (IO.Directory.GetDirectories(repo.gitdir).Length <> 0)
    then
      failwith $"{Repository(Directory_is_not_empty(repo.gitdir))}"
    else
      repo
  else
    IO.Directory.CreateDirectory(repo.worktree) |> ignore
    repo

let check_files (repo : repository) =
  Path.repo_dir (repo.gitdir, [| "branches" |], true) |> ignore
  Path.repo_dir (repo.gitdir, [| "objects" |], true) |> ignore
  Path.repo_dir (repo.gitdir, [| "refs"; "tags" |], true) |> ignore
  Path.repo_dir (repo.gitdir, [| "refs"; "heads" |], true) |> ignore

  repo

let init_files (repo : repository) =
  use description =
    new IO.StreamWriter(Path.repo_file (repo.gitdir, [| "description" |], true))

  use head = new IO.StreamWriter(Path.repo_file (repo.gitdir, [| "HEAD" |], true))
  let config = new FileIniDataParser()

  description.Write(
    "Unnamed repository; edit this file 'description' to name the repository.\n"
      .ToCharArray()
  )

  head.Write("ref: refs/heads/main\n")
  use write = new IO.StreamWriter(repo.conf)
  config.WriteData(write, DEFAULT_CONFIG)
  repo

/// Create a new repository at path
let init (path : string) =

  check_path (path) |> check_files |> init_files
