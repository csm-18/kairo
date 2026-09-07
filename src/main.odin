package main

import "cli"
import "core:os"

main :: proc() {
	args := os.args[1:]
	cli.run(args)
}
