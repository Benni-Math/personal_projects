cmake_minimum_required(VERSION 3.8)

include_directories(/usr/local/include)
include_directories(/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include)

include_directories(.)
include_directories(calcser)
include_directories(calcsvc)
include_directories(server/srvcore)
include_directories(client/clicore)

link_directories(/usr/local/lib)
link_directories(/opt/homebrew/lib)
link_directories(/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/lib)

add_compile_options("-g")

add_subdirectory(calcser)
add_subdirectory(calcsvc)
add_subdirectory(server)
add_subdirectory(client)

