// An example of using command-line arguments
package main

import (
    "fmt"
    "os"
)

func main() {
    // using short variable declaration
    s, sep := "", ""
    // iterating over a range with for
    for _, arg := range os.Args[1:] {
        s += sep + arg
        sep = " "
    }
    fmt.Println("Here is your input:")
    fmt.Println(s)

    // We could also just use the built-in:
    // fmt.Println(strings.Join(os.Args[1:], " "))
    //
    // or, if we don't care about format:
    // fmt.Println(os.Args[1:])
}

func first_iteration() {
    var s, sep string
    for i := 1; i < len(os.Args); i++ {
        s += sep + os.Args[i]
        sep = " "
    }
    fmt.Println("Here is your input:")
    fmt.Println(s)
}

func exercise1_1() {
    fmt.Println(os.Args[0])
}

func exercise1_2() {
    for index, arg := range os.Args[1:] {
        fmt.Println(index, ":", arg)
    }
}

