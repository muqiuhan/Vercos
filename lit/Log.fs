module Lit.Log

open Logary
open Logary.Message
open Logary.Configuration
open Logary.Targets
open Hopac

let logary =
  Config.create "Logary.ConsoleApp" "laptop"
  |> Config.target (LiterateConsole.create LiterateConsole.empty "console")
  |> Config.ilogger (ILogger.Console Debug)
  |> Config.build
  |> run

let logger = logary.getLogger "Lit"

let Info (msg : string) = logger.info (eventX msg)
let Debug (msg : string) = logger.debug (eventX msg)
let Warn (msg : string) = logger.warn (eventX msg)
let Error (msg : string) = logger.error (eventX msg)

let Fatal (msg : string) =
  logger.fatal (eventX msg)
  exit 1
