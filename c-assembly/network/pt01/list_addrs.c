/**
*   list_addr.c: uses ifaddrs/getifaddrs to list all IP addresses in use
*/

#include "net_header.h"

int main() {
    
    // creating pointer to ifaddrs by name of addresses
    struct ifaddrs *addresses;

    // getifaddr allocates memory and returns a linked list of addrs
    // returns -1 if there is an error
    if (getifaddrs(&addresses) == -1) {
        printf("getifaddrs call failed\n");
        return -1;
    }

    // Just an alias (for readability)?
    //  - no, we need to keep `addresses` so we can free it later (:50)
    struct ifaddrs *address = addresses;
    // Next, we while loop through the linked list
    // and print the individual addresses (with additional info)
    while(address) {
        // This is a check for an empty list (and for empty nodes)
        if (address->ifa_addr == NULL) {
            address = address->ifa_next;
            continue;
        }

        // Getting whether the address is IPv4 of IPv6
        int family = address->ifa_addr->sa_family;
        if (family == AF_INET || family == AF_INET6) {
            
            printf("%s\t", address->ifa_name);
            printf("%s\t", family == AF_INET ? "IPv4" : "IPv6");
            
            char ap[100];
            const int family_size = family == AF_INET ?
                sizeof(struct sockaddr_in) : sizeof(struct sockaddr_in6);
            getnameinfo(address->ifa_addr,
                family_size, ap, sizeof(ap), 0, 0, NI_NUMERICHOST);
            printf("\t%s\n", ap);
        }
        // ifa_next returns 0 when we ar at the end of the linked list
        address = address->ifa_next;
    }

    // We need to manually free the linked list at the end
    freeifaddrs(addresses);
    return 0;
}
