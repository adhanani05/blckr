# blckr

Simple, fast CLI to block and unblock websites on Linux by editing `/etc/hosts`. Great for quick focus sessions without heavyweight tooling.

## Install

- From crates.io (recommended):
  - `cargo install blckr`

- From source (repo):
  - `cargo build --release`
  - The binary is at `target/release/blckr`

## Quick Start

- Block a domain:
  - `sudo blckr block example.com`

- Unblock a domain:
  - `sudo blckr unblock example.com`

Root privileges are required because the tool writes to `/etc/hosts`.

## Usage

- Syntax:
  - `blckr <action> <domain>`
- Actions:
  - `block`: Append entries to `/etc/hosts` for the domain
  - `unblock`: Remove those exact entries if present
- Examples:
  - `sudo blckr block twitter.com`
  - `sudo blckr block www.youtube.com`
  - `sudo blckr unblock twitter.com`
  - `sudo blckr unblock www.youtube.com`

Note: The tool operates on one domain per invocation. To handle multiple domains, run the command multiple times.

## How It Works

- File: `/etc/hosts`
- On block:
  - Appends two lines to the end of `/etc/hosts` if the domain string isn’t already present anywhere in the file:
    - `0.0.0.0 <domain>`
    - `::1 <domain>`
- On unblock:
  - Removes lines that exactly match the two entries above (including the trailing newline).
- Idempotency:
  - Block won’t append duplicate lines if the domain string already exists anywhere in `/etc/hosts` (even if the prior line wasn’t added by blckr).
  - Unblock only removes the two exact lines described above.

## Notes and Limitations

- Linux-only: The path is hardcoded to `/etc/hosts`. It may also work on other Unix-like systems, but only Linux is targeted.
- Exact matches only: Unblock removes only the exact lines it previously adds. If you manually change formatting or spacing, unblock may not remove them.
- No wildcard/subdomain handling: Blocking `example.com` does not automatically block `www.example.com` (or vice versa). Add each variant you want to block.
- No backup: The tool does not create a backup of `/etc/hosts`. Consider making one:
  - `sudo cp /etc/hosts /etc/hosts.backup`
- DNS caching: Many Linux setups apply `/etc/hosts` immediately, but if you use a caching resolver, you may need to flush:
  - systemd-resolved: `sudo resolvectl flush-caches`
  - nscd: `sudo systemctl restart nscd`
  - NetworkManager (rarely needed): `sudo systemctl restart NetworkManager`

## Troubleshooting

- Command not found:
  - Ensure `~/.cargo/bin` is in your `PATH` or call the full path after `cargo install blckr`.
- Permission denied:
  - Use `sudo` for `block`/`unblock` since `/etc/hosts` requires root.
- Domain still resolves:
  - Flush DNS caches (see above).
  - Ensure the domain you blocked is the one you’re testing (e.g., also block `www.` variants).
- Unblock didn’t remove lines:
  - If lines were edited manually (e.g., extra spaces), remove them by hand from `/etc/hosts`.

- Check if `curl -I https://example.com` still connects:
  - Check resolution: `getent hosts example.com`. If this shows real IPs (not `0.0.0.0` or `::1`), `/etc/hosts` isn’t being applied.
  - Ensure `/etc/nsswitch.conf` has `hosts: files dns` (or `files` before `dns`) so the hosts file is consulted first.
  - Flush caches: `sudo resolvectl flush-caches`.

- Browser still loads site:
  - Disable DNS over HTTPS (DoH) in browser network settings:
    - Firefox: Settings > General > Network Settings > disable DNS over HTTPS.
    - Chrome/Chromium: Settings > Privacy and security > Security > toggle off “Use secure DNS”. Some Chrome-based browsers place this under their own network settings.
  - Disable VPNs or enterprise resolvers that bypass `/etc/hosts`.
  - Clear the browser’s DNS cache and restart the browser.

## Uninstall

- If installed from crates.io:
  - `cargo uninstall blckr`
- If built from source:
  - Remove the built binary (e.g., `target/release/blckr`) or any copied installation path.
- Hosts cleanup:
  - Run `sudo blckr unblock <domain>` for each domain you previously blocked, or manually remove those lines from `/etc/hosts`.

## Development

- Build:
  - `cargo build --release`
- Run (writes to `/etc/hosts`):
  - `sudo cargo run -- block example.com`
  - `sudo cargo run -- unblock example.com`

## Contributing

- Issues and PRs are welcome.
- Please keep changes minimal and focused, and ensure behavior remains predictable and idempotent.

## License

MIT
