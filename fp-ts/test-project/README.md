# Functional Programming with TS
##  playground & Test Website

Front-end is a basic functional JS implementation with jquery and ramda, which will later be upgraded to TS and other stuff.

Currently, calls the Flickr API to get pictures of english bulldogs.

Back-end is a handwritten Rust server, which only uses standard library TCP tools.

To run, just use the usual `cargo run` command -- to change the Flickr API call, go to client/main.js and change the last line of `app('english bulldogs')` to 'app(`whatever you want to call')` and the Flickr API will return photos matching the  search for those keywords.
