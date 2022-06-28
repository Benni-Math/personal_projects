// fetch prints the content found at a URL
package main

import (
    "fmt"
    "strings"
    "net/http"
    "os"
    "io"
)

func main() {
    for _, url := range os.Args[1:] {
        fullurl := validUrl(url) 
        resp, err := http.Get(fullurl)
        if err != nil {
            fmt.Fprintf(os.Stderr, "fetch: %v\n", err)
            os.Exit(1)
        }

        fmt.Printf("Status: %s\n", resp.Status)

        _, err = io.Copy(os.Stdout, resp.Body)
        resp.Body.Close() // don't leak resources
        if err != nil {
            fmt.Fprintf(os.Stderr, "fetch: reading %s: %v\n", url, err)
            os.Exit(1)
        }
    }
}

// Function to validate the URL
// and return a well-formatted url
// This isn't fully implemented (it's very basic)
func validUrl(url string) string {
    var fullurl string
    if strings.HasPrefix(url, "http://") {
        fullurl = url
    } else {
        fullurl = "http://" + url
    }
    return fullurl
}

