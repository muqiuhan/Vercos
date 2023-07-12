module Vercos.Repository

open System
open System.IO
open Log
open IniParser

/// A vercos repository
type Repository(__path, __force) =
    let __vercosDir = Path.Join([| __path; ".vercos" |])
    let __worktree = __path

    do
        if not (__force || Directory.Exists(__vercosDir)) then
            Log.Error $"{__path} is not a Vercos repository"

    let __configFile = FileIniDataParser()

    /// Compute path under repo's vercosdir
    member private this.RepoPath(path: array<string>) =
        Path.Join(Array.append [| __vercosDir |] path)


    /// Same as RepoPath, but mkdir path if absent if mkdir
    member private this.RepoDir(path: array<string>, mkdir: bool) =
        let path: string = this.RepoPath(path)

        if Path.Exists(path) then
            if Directory.Exists(path) then
                Some(path)
            else
                Log.Error $"{path} is not a directory"
        else if mkdir then
            Directory.CreateDirectory(path) |> ignore
            Some(path)
        else
            None

    /// Same as RepoPath, but creatre directory isf absent.
    /// For example:
    ///     RepoFile([|"refs"; "remotes"; "origin"; "HEAD"|])
    /// will create ".vercos/refs/remotes/origin"
    member private this.RepoFile(path: array<string>, mkdir: bool) =
        let path: array<string> = Array.sub path 0 (path.Length - 1)

        if this.RepoDir(path, mkdir).IsSome then
            Some(this.RepoPath(path))
        else
            None

    /// Make sure the path either doesn't exist or is an empty dir and create it
    member private this.CheckCreateEnvironments(path: string) =
        if Path.Exists(__worktree) then
            if not (Directory.Exists(__worktree)) then
                Log.Error $"{path} is not a directory"

            if Directory.GetDirectories(__worktree).Length = 0 then
                Log.Error $"{path} is not empty!"

        else
            Directory.CreateDirectory(__worktree) |> ignore

    /// Create vercos directory tree and verify that it was created successfully
    member private this.CreateVercosTreeAndCheckIt() =
        assert (this.RepoDir([| "branches" |], true).IsSome)
        assert (this.RepoDir([| "objects" |], true).IsSome)
        assert (this.RepoDir([| "refs"; "tags" |], true).IsSome)
        assert (this.RepoDir([| "refs"; "heads" |], true).IsSome)
        assert (this.RepoFile([| "config" |], true).IsSome)

    /// Create vercos initial file information
    member private this.CreateInitialFileInfo() =
        // .vercos/description
        use description = File.OpenWrite(Path.Join([| __vercosDir; "description" |]))

        // .vercos/HEAD
        use HEAD = File.OpenWrite(Path.Join([| __vercosDir; "HEAD" |]))

        description.Write(
            Text.Encoding.UTF8.GetBytes("Unnamed repository; edit this file 'description' to name the repository.\n")
        )

        HEAD.Write(Text.Encoding.UTF8.GetBytes("ref: refs/heads/master\n"))

    /// Create vercos config file
    member private this.CreateConfigFile() =
        use config = File.OpenWrite(Path.Join([| __vercosDir; "config" |]))

        config.Write(
            Text.Encoding.UTF8.GetBytes(
                """[core]
    repositoryformatversion = 0
    filemode = false
    bare = false
"""
            )
        )


    /// Create a new repository at path
    member public this.Create() =
        Log.Info $"Create a repository on `{__path}` ..."

        this.CreateVercosTreeAndCheckIt()
        |> this.CreateInitialFileInfo
        |> this.CreateConfigFile

        Log.Info $"The repository `{__path}` is created!"
