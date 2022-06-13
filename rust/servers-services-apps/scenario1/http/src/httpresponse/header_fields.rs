// Just of const of all the standard HTTP response header fields
// TODO: possibly upgrade this to an enum with a set of methods
// and ways of 'extracting' what each header field means (corresponding logic)
use std::collections::HashSet;

const HEADER_ARRAY: &'static [str] = [
  // Request HTTP Client Hints
  "Accept-CH",
  // Specifying which web sites can participate in cross-origin resource sharing
  "Access-Control-Allow-Origin",
  "Access-Control-Allow-Credentials",
  "Access-Control-Expose-Headers",
  "Access-Control-Max-Age",
  "Access-Control-Allow-Methods",
  "Access-Control-Allow-Headers",
  // Specifies which patch document formats this server supports
  "Accept-Path",
  // What partial content range types this server supports via byte serving
  "Accept-Ranges",
  // The age the object has been in proxy cache in seconds
  "Age",
  // Valid methods for a specified resource
  // To be used for a '405 Method not allowed'
  "Allow",
  // Indicates that resources can be accessed at a different network location
  "Alt-Svc",
  // Tells all caching mechanisms from server to client whether they may cache this object
  "Cache-Control",
  // Control options for the current connection
  "Connection",
  // An opportunity to raise a 'File Download' dialogue box
  "Content-Disposition",
  // Type of encoding used on the data, see HTTP compression
  "Content-Encoding",
  // The languages of the intended audience for the enclosed content
  "Content-Language",
  // The length of the response body in octets (8-bit bytes)
  "Content-Length",
  // An alternate location for the returned data
  "Content-Location",
  // A Base64-encoded binary MD5 sum of the content of the response
  "Content-MD5",
  // Where in a full body message this partial message belongs
  "Content-Range",
  // The MIME type of this content
  "Content-Type",
  // The date and time that the message was sent
  "Date",
  // Specifies the delta-encoding entity tag of the response
  "Delta-Base",
  // An identifier for a specific version of a resource, often a message digest
  "ETag",
  // Gives the date/time after which the response is considered stale
  "Expires",
  // Instance manipulations applied to the response
  "IM",
  // The last modified date for the requested object
  "Last-Modified",
  // Used to express a typed relationship with another resource
  "Link",
  // Used in redirection, or when a new resource has been created
  "Location",
  // Set P3P policy
  "P3P",
  // Implementation specific fields (various effects)
  "Pragma",
  // Indicates which "Prefer" tokens were honored by the server
  "Preference-Applied",
  // Request an authentication to access the proxy
  "Proxy-Authenticate",
  // HTTP Public Key Pinning, announces hash of website's authentic TLS certificate
  "Public-Key-Pins",
  // If an entity is temporarily unavailable, this instructs the client to try again later
  // Value can be a specified period of time (in seconds) or a HTTP-date
  "Retry-After",
  // A name for the server
  "Server",
  // An HTTP cookie
  "Set-Cookie",
  // A HSTS Policy informing the HTTP client how long to cache the HTTPS only policy
  // and whether this applies to subdomains
  "Strict-Transport-Security",
  // Indicates that the given set of header fields is present
  // in the trailer of a message encoded with chunked transfer encoding
  "Trailer",
  // The form of encoding used to safely transfer the entity to the user
  // Currently defined methods: chunked, compress, deflate, gzip, identity
  "Transfer-Encoding",
  // Tracking status header
  "Tk",
  // Ask the client to upgrade to another protocol
  "Upgrade",
  // Tells downstream proxies how to match future request headers to decide whether the
  // cached response can be used rather than requesting a fresh one from the origin server
  "Vary",
  // Informs the client of proxies through which the response was sent
  "Via",
  // A general warning about possible problems with the entity body
  "Warning",
  // Indicates the authentication scheme that should be used to access the requested entity
  "WWW-Authenticate",
  // Clickjacking protection
  "X-Frame-Options",
];

pub const HEADER_FIELDS: &'static HashSet<&str> = HashSet::from(HEADER_ARRAY);