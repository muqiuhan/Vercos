module Vercos.Log

#nowarn "3535"

open System

module Logger =

    [<Interface>]
    type T =
        abstract member Info: String -> Unit
        abstract member Warn: String -> Unit
        abstract member Error: String -> Unit
        abstract member Debug: String -> Unit

    type Console private () =

        static let self: Console = Console()

        /// Output logs with additional colors, the output will be locked
        static member private log (color: ConsoleColor) (s: String) : Unit =
            let consoleLogOutputLock = obj ()

            lock consoleLogOutputLock (fun _ ->
                Console.ForegroundColor <- ConsoleColor.White
                printf $"""{(DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"))}"""
                Console.ForegroundColor <- color
                printf " > "
                Console.ResetColor()
                printfn $"{s}")

        interface T with
            member this.Info s = Console.log ConsoleColor.Green s
            member this.Debug s = Console.log ConsoleColor.Cyan s
            member this.Warn s = Console.log ConsoleColor.Yellow s
            member this.Error s = Console.log ConsoleColor.Red s

        static member Info = (self :> T).Info
        static member Debug = (self :> T).Debug
        static member Warn = (self :> T).Warn
        static member Error = (self :> T).Error

type Log private () =

    static member Info(message: String) : Unit = Logger.Console.Info message

    static member Debug(message: String) : Unit = Logger.Console.Debug message

    static member Warn(message: String) : Unit = Logger.Console.Warn message

    static member Error(message: String) =
        Logger.Console.Error message
        failwith $"{message}"
