package cli

import "core:fmt"

run :: proc(args: []string) {
	fmt.println(len(args))

}
