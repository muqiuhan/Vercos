module Vercos.Cli

open Argu
open System

type Args =
    | [<EqualsAssignment>] Init of path: string option

    interface IArgParserTemplate with
        member this.Usage =
            match this with
            | Init _ -> "Initialize a new repository"


type Cli() =
    static member private parser =
        ArgumentParser.Create<Args>(
            programName = "vercos",
            errorHandler =
                ProcessExiter(
                    colorizer =
                        function
                        | ErrorCode.HelpText -> None
                        | _ -> Some ConsoleColor.Red
                )
        )

    static member public parse(args: array<string>) =
        let args = Cli.parser.Parse(args)

        if args.Contains(Init) then
            args.GetResult(Init) |> ignore
        else
            ()
