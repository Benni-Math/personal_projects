//go:build ignore

package main

import (
    "html/template" 
    "os"
    "log"
    "net/http"
    "regexp"
    "fmt"
)

type Page struct {
    Title string
    Body []byte
}

func (p *Page) save() error {
    filename := p.Title + ".txt"
    // os.WriteFile returns an error if it fails
    return os.WriteFile(filename, p.Body, 0600)
}

// Bad first version (not handling errors)
// func loadPage(title string) *Page {
//     filename := title + ".txt"
//     body, _ := os.ReadFile(filename)
//     return &Page{Title: title, Body: body}
// }

func loadPage(title string) (*Page, error) {
    filename := title + ".txt"
    body, err := os.ReadFile(filename)
    if err != nil {
        return nil, err
    }
    return &Page{Title: title, Body: body}, nil
}

// Our main function (so far)
// is just testing this data structure
// func main() {
//     p1 := &Page{Title: "TestPage", Body: []byte("This is a sample Page.")}
//     p1.save()
//     p2, _ := loadPage("TestPage")
//     fmt.Println(string(p2.Body))
// }

// Adding html templating tools with html/template

// Old un-cached version of our renderTemplate function 
// func renderTemplate(w http.ResponseWriter, tmpl string, p *Page) {
//     t, err := template.ParseFiles(tmpl + ".html")
//     if err != nil {
//         http.Error(w, err.Error(), http.StatusInternalServerError)
//         return
//     }
// 
//     err = t.Execute(w, p)
//     if err != nil {
//         http.Error(w, err.Error(), http.StatusInternalServerError)
//     }
// }

// Global variable for template caching
var templates = template.Must(template.ParseFiles("edit.html", "view.html"))

// The main rendering function for our html templates
func renderTemplate(w http.ResponseWriter, tmpl string, p *Page) {
    err := templates.ExecuteTemplate(w, tmpl+".html", p)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
    }
}

// Using regexp to validate URL paths
var validPath = regexp.MustCompile("^/(edit|save|view)/([a-zA-Z0-9]+)$")

// Adding a makeHandler function (for currying getTitle)
func makeHandler(fn func (http.ResponseWriter, *http.Request, string)) http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        m := validPath.FindStringSubmatch(r.URL.Path)
        if m == nil {
            http.NotFound(w, r)
            return
        }
        fn(w, r, m[2])
    }
}

// Adding in the server functionality with net/http

// Serves up the HTML of the specified wiki page
func viewHandler(w http.ResponseWriter, r *http.Request, title string) {
    p, err := loadPage(title)

    // if someone tries to view a non-existent page
    // send them to an edit page for the page
    // (so they end up creating the new page)
    if err != nil {
        http.Redirect(w, r, "/edit/"+title, http.StatusFound)
        return
    }
    
    renderTemplate(w, "view", p)
}

// Used to edit wiki pages
func editHandler(w http.ResponseWriter, r *http.Request, title string) {
    p, err := loadPage(title)
    if err != nil {
        p = &Page{Title: title}
    }

    renderTemplate(w, "edit", p)
}

// Used to save wiki pages
func saveHandler(w http.ResponseWriter, r *http.Request, title string) {
    // Getting the info from the form request
    body := r.FormValue("body")
    // Saving it into a Page struct
    p := &Page{Title: title, Body: []byte(body)}

    // Error handling the .save()
    err := p.save()
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    
    http.Redirect(w, r, "/view"+title, http.StatusFound)
}

func testHello(w http.ResponseWriter, r *http.Request) {
    msg := "world"
    fmt.Fprintf(w, "Hello %s", msg)
}

// New main which starts up the web server
func main() {
    http.HandleFunc("/", testHello)

    http.HandleFunc("/view/", makeHandler(viewHandler))
    http.HandleFunc("/edit/", makeHandler(editHandler))
    http.HandleFunc("/save/", makeHandler(saveHandler))

    log.Fatal(http.ListenAndServe(":8080", nil))
}

