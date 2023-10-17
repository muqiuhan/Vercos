<div align="center">

<img src=".github/logo.png" height="150px" width="150px">

# Lit

*A __minimalism__ __cross-platform__ __git-like__ version control system written in F#*

![](https://img.shields.io/badge/.NET%20Core%208.0.100~rc.2-8A2BE2)
![](https://github.com/muqiuhan/lit/actions/workflows/build.yml/badge.svg)

![](https://img.shields.io/badge/work%20in%20progress-FFFF00)

</div>

## Introduction
This is not a new Version Control System. It simplifies many features and codes based on the current git,
which can be used for teaching and research, or secondary development based on this project.

The goal is simple, fast, cross-platform and beautiful command line output.

## Build and Publish

This project enables .NET Native AOT construction by default:
```xml
<PublishAOT>true</PublishReadyAOT>
<PublishTrimmed>true</PublishTrimmed>
```

Just: `dotnet publish -c Release -r [Your Platform]`
> E.g: `dotnet publish -c Release -r win-x64`

## Usage

```
USAGE: lit [version] [help] [add] [init] [log] [rm] [tagging] [status] [cat-file] [check-ignore] [checkout] [commit] [hash-object] [ls-files] [ls-tree] [rev-parse] [show-ref]

OPTIONS:

    version               Display version information about lit
    help                  Display help information about lit
    add                   Add file contents to the index
    init                  Create an empty lit repository or reinitialize an existing one
    log                   Show commit logs
    rm                    Remove files from the working tree and from the index
    tagging               Create, list, delete or verify a tag object signed with GPG
    status                Show the working tree status
    cat-file              Provide content or type and size information for repository objects
    check-ignore          Debug gitignore / exclude files
    checkout              Switch branches or restore working tree files
    commit                Record changes to the repository
    hash-object           Compute object ID and optionally create an object from a file
    ls-files              Show information about files in the index and the working tree
    ls-tree               List the contents of a tree object
    rev-parse             Pick out and massage parameters
    show-ref              List references in a local repository
```

...

## Acknowledgements
- [ini-parser](https://github.com/rickyah/ini-parser): For parsing configuration files
- [Argu](https://github.com/fsprojects/Argu): For parsing command line arguments

## LICENSE
The MIT License (MIT)

Copyright (c) 2023 Muqiu Han

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.