# Implementing TCP in Rust
## Following along with Jon Gjengset's video series

**TODO**: Since the Rust `tun_tap` library only works for Linux, I will need to
create a Docker container to test this.

---

## Notes:
 - [TUN/TAP](https://www.kernel.org/doc/Documentation/networking/tuntap.txt) is
   a way for us to simulate ourselves as a network host. Essentially, when the
   TUN/TAP is active, a corresponding IP address by the name of tun<num> will
   be visible to the kernel -- there will be an exposed interface in userspace
   that will allow us to send TCP messages to the kernel and for the kernel to
   respond accordingly.
    - To implement TUN/TAP we will use the crate
      [`tun_tap`](https://docs.rs/tun-tap/latest/tun_tap/) which allows
      us to create an `Iface` object to `send` and `recv` with the TUN/TAP
    - Unfortunately, we will also need to implement an IP protocol (kinda easy)

 - For the Docker container, we will extend the generic Rust
   [image](https://hub.docker.com/_/rust) 
    - Currently having issues with musl
    - Keep getting 'is `aarch64-linux-musl-gcc` installed?'

