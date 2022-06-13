# Notes on servers
## (implemented in Rust)

Rust has a built in TCP library in std::net, but TCP is only a way of reading and writing byte streams meaning it is just a 'transport' method (for more details, read about OSI and the TCP/IP protocol stack).

There are four components to the basic web server that we will be designing, each with a specifici purpose, in line with the *Single Responsibility Principle* (SRP).
 1. The **Server** listens for incoming TCP byte streams.
 2. The **HTTP library** interprets byte streams and converts it to *HTTP request*.
 3. The **router** accepts an HTTP request and determines which handler to invoke.
 4. The **handler** processes the HTTP request and constructs and *HTTP response*.
This response is then converted back to byte stream using the HTTP library, which is then sent back to the client using the TCP server.

### Server
This is the 'easiest' one, since we can use Rust's built in TcpListener. There is already a basic implemenation in place, but once we finish the other tools we'll 'beef it up'.

### HTTP library
First up, for the HTTP request, we need a couple data structures:
 1. HttpRequest - a struct which represents the HTTP request
 2. Method - an enum which specifies the allowed values/variants for HTTP Methods
 3. Version - an enum which specifies the allow HTTP Versions

Version and Method are rather self-explanatory (and for now we will only implement GET and POST for HTTP version 1.1). The real meat is the HttpRequest struct.

The structure of an HTTP request is simple (all lines separated by "/r/n"):
 - First, a *request line* which contains the method, path, and version
    - These will be extracted into HttpRequest.Method, .Resource, and .Version, respectively
 - Next, there are the separate *header lines* 
    - These have *keys*, e.g. "Host", "User-Agent", etc.
    - And "values" corresponding to the keys
    - So, this will be saved as a HashMap, called HttpRequest.headers
 - Finally, there is an optional *message body* which is separated from the rest of the request by an empty line, i.e. "/r/n/r/n" - this will be simply stored as a String called HttpRequest.msg_body

As such, the main function for the httprequest.rs file is the `impl From<String> for HttpRequest` 

