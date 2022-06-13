// Just of const of all the standard HTTP request header fields
// TODO: possibly upgrade this to an enum with a set of methods
// and ways of 'extracting' what each header field means (corresponding logic)
use std::collections::HashSet;

const HEADER_ARRAY: &'static [str] = [
  // Acceptable instance-manipulations for the request
  "A-IM",
  // Media type(s) that is/are acceptable for the response
  "Accept",
  // Character sets that are acceptable
  "Accept-Charset",
  // Acceptable version in time
  "Accept-Datetime",
  // List of acceptable encodings (HTTP compression)
  "Accept-Encoding",
  // List of acceptable human languages for response
  "Accept-Language",
  // Initiates a request for cross-origin resource sharing with "Origin"
  "Access-Control-Request-Method",
  "Access-Control-Request-Headers",
  // Authentication credentials for HTTP authentication
  "Authorization",
  // Specifies directives that must be obeyed by all caching mechanisms along the req-res chain
  "Cache-Control",
  // Control options for the current connection
  "Connection",
  // Type of encoding used on the data
  "Content-Encoding",
  // Length of the request body in octets (8-bit bytes)
  "Content-Length",
  // A Base64-encoded binary MD5 sum of the content of the request body
  "Content-MD5",
  // The Media type of the body of the request (used with POST & PUT)
  "Content-Type",
  // An HTTP cookie previously sent by the server with "Set-Cookie"
  "Cookie",
  // The date and time at which the message was originated
  "Date",
  // Indicates that particular server behaviors are required by the client
  "Expect",
  // Disclose original information of a client connecting to a web server through an HTTP proxy
  "Forwarded",
  // The email address of the user making the request
  "From",
  // Domain name of the server and the TCP port number on which the server is listening
  "Host",
  // A request that upgrades from HTTP/1.1 to HTTP/2
  "HTTP2-Settings",
  // (For PUT) only perform the action if the client supplied entity matches the same entity on the server
  "If-Match",
  // Allows a '304 Not Modified' to be returned if content is unchanged
  "If-Modified-Since",
  // Similar to above -- see HTTP ETag
  "If-None-Match",
  // If the entity is unchanged, send me the parts that I am missing; otherwise, send me the entire new entity
  "If-Range",
  // Only send the response if the entity has not been modified since a specific time
  "If-Unmodified-Since",
  // Limit the number of times the message can be forwarded through proxies or gateways
  "Max-Forwards",
  // Initiates a request for cross-origin resource sharing
  // (ask server for "Access-Control-*" response fields)
  "Origin",
  // Implementation-specific fields (various effects)
  "Pragma",
  // Allows client to request that certain behaviors be employed by a server while processing a request
  "Prefer",
  // Authorization credentials for connecting to a proxy
  "Proxy-Authorization",
  // Request only part of an entity
  "Range",
  // The address of the previous web page from which a link to the currently requested page was followed
  "Referer",
  // Transfer encodings the user agent is willing to accept
  "TE",
  // Indicates that the given set of header fields is present in the trailer of a message encoded with chunked transfer coding
  "Trailer",
  // Form of encoding used to safely transfer the entity to the user
  // Currently defined methods: chunked, compress, deflate, gzip, identity
  "Transfer-Encoding",
  // The user agent string of the user agent
  "User-Agent",
  // Ask the server to upgrade to another protocol
  "Upgrade",
  // Informs the server of proxies through which the request was sent
  "Via",
  // A general warning about possibly problems with the entity body
  "Warning",
];

pub const HEADER_FIELDS: &'static HashSet<&str> = HashSet::from(HEADER_ARRAY);