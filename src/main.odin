package main

import "core:fmt"
import "core:os"

main :: proc(){
    args := os.args[1:]
    fmt.println(len(args))
}