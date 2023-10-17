module Lit.Errors

type Errors =
  | Repository of Repository
  | Unknown

  override this.ToString () =
    match this with
    | Repository repository ->
      match repository with
      | NotLitRepository path -> $"{path} is not a lit repository"
      | NotDirectory path -> $"{path} is not a directory"
      | ConfigFileMissing path -> $"{path} is missing"
    | Unknown -> "Unknown"

and Repository =
  | NotLitRepository of string
  | NotDirectory of string
  | ConfigFileMissing of string
  | UnsupportedRepositoryFormatVersion of string