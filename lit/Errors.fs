module Lit.Errors

type Errors =
  | Repository of Repository
  | Unknown

  override this.ToString () =
    match this with
    | Repository repository ->
      match repository with
      | Not_lit_repository path -> "{path} is not a lit repository"
      | Not_directory path -> $"{path} is not a directory"
      | Config_file_missing path -> $"{path} is missing"
      | Unsupported_repository_format_version version ->
        "Unsupported repositoryformatversion ${version}"
      | Directory_is_not_empty path -> $"{path} is not empty"
    | Unknown -> "Unknown"

and Repository =
  | Not_lit_repository of string
  | Not_directory of string
  | Config_file_missing of string
  | Unsupported_repository_format_version of string
  | Directory_is_not_empty of string
