module Vercos.Repository

open System
open System.IO
open System.IO.Compression

open Log
open IniParser
open Basic

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

    /// Find the root of the current repository
    static member public Find(path: string, required: bool) =
        let path = Path.GetFullPath(path)
        let parent = Path.GetFullPath(Path.Join([| path; ".." |]))

        if Directory.Exists(Path.Join([| path; ".vercos" |])) then
            Some(Repository(path, false))
        else if parent = path then
            if required then Log.Error "No git directory" else None
        else
            Repository.Find(parent, required)


    /// Create a new repository at path
    member public this.Create() =
        Log.Info $"Create a repository on `{__path}` ..."

        this.CreateVercosTreeAndCheckIt()
        |> this.CreateInitialFileInfo
        |> this.CreateConfigFile

        Log.Info $"The repository `{__path}` is created!"

    member private this.ReadSize(raw: string) =
        let size = System.Collections.Generic.List<char>()

        try
            for c in raw do
                if Char.IsDigit c then size.Add(c) else raise Break

            String.Join("", size)
        with :? Break ->
            String.Join("", size)

    /// Read object id from vercos repository.
    /// Return a Object whose exact type depends on the object.
    member public this.Read(repo: Repository, sha: string) =
        let raw =
            (new StreamReader(
                new ZLibStream(
                    File.OpenRead(
                        this
                            .RepoFile(
                                [| "objects"; (sha.Substring(0, 2)); (sha.Substring(2, (sha.Length - 2))) |],
                                false
                            )
                            .Value
                    ),
                    CompressionMode.Decompress
                )
            ))
                .ReadToEnd()

        let x = raw.IndexOf(" ")
        let y = raw.IndexOf("\x00", x)

        let fmt = raw.Substring(0, x)
        let size = int (raw.Substring(x + 1) |> this.ReadSize)

        if size <> (raw.Length - y - 1) then
            Log.Error $"Malformed object {sha}: bad length"

        printfn $"{fmt}"
