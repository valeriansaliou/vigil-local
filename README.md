Vigil Local
===========

[![Test and Build](https://github.com/valeriansaliou/vigil-local/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/valeriansaliou/vigil-local/actions?query=workflow%3A%22Test+and+Build%22) [![Build and Release](https://github.com/valeriansaliou/vigil-local/workflows/Build%20and%20Release/badge.svg)](https://github.com/valeriansaliou/vigil-local/actions?query=workflow%3A%22Build+and+Release%22) [![dependency status](https://deps.rs/repo/github/valeriansaliou/vigil-local/status.svg)](https://deps.rs/repo/github/valeriansaliou/vigil-local) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

**Vigil Local daemon. Used as a slave service to monitor hosts behind a firewall and report their status to Vigil.**

Vigil Local is an (optional) slave daemon that you can use to report internal service health to your [Vigil-powered status page](https://github.com/valeriansaliou/vigil) master server. It is designed to be used behind a firewall, and _**to monitor hosts bound to a local loop or LAN network, that are not available to your main Vigil status page**_. It can prove useful as well if you want to fully isolate your Vigil status page from your internal services.

Install Vigil Local on a server of yours and configure it with your Vigil endpoint URL and token; it will then start monitoring all configured nodes and report them to Vigil. Make sure that you pre-configure all local nodes as `local` in Vigil, and then as `poll` or `script` in Vigil Local, accordingly. The service identifier and node identifier must match on either sides, as they will be used to identify the replica status being reported from the Vigil Local slave to the Vigil master.

Multiple slave daemons can run on separate servers or networks, and report a group of services and nodes to the same Vigil master. Make sure that multiple slaves are not double-reporting replicas on the same monitored service/node pair.

_Tested at Rust version: `rustc 1.91.1 (ed61e7d7e 2025-11-07)`_

**🇧🇬 Crafted in Sofia, Bulgaria.**

## What is Vigil?

[Vigil is an open-source Status Page](https://github.com/valeriansaliou/vigil) you can host on your infrastructure, used to monitor all your servers and apps, and visible to your users.

It lets you monitor your critical systems using a variety of methods: `push` for applications using a Vigil Reporter library, `poll` for Vigil-reachable HTTP, TCP & ICMP services, `script` to execute custom probes, and `local` for non-Vigil-reachable private services.

Vigil Local lets you monitor nodes that are configured in `local` mode (in Vigil), aside other monitoring methods that do not require the Vigil Local utility (and thus can run on Vigil itself right away).

## How to use it?

### Installation

**Install from binary:**

A pre-built binary of Vigil Local is shared in the releases on GitHub. You can simply download the latest binary version from the [releases page](https://github.com/valeriansaliou/vigil-local/releases), and run it on your server.

You will still need to provide the binary with the configuration file, so make sure you have a Vigil Local `config.cfg` file ready somewhere.

_The binary provided is statically-linked, which means that it will be able to run on any Linux-based server. Still, it will not work on MacOS or Windows machines._

👉 _Each release binary comes with an `.asc` signature file, which can be verified using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc)._

**Install from packages:**

Vigil Local provides [pre-built packages](https://packagecloud.io/valeriansaliou/vigil-local) for Debian-based systems (Debian, Ubuntu, etc.).

**Important: Vigil Local only provides 64 bits packages targeting Debian 11 & 12 for now (codenames: `bullseye` & `bookworm`). You will still be able to use them on other Debian versions, as well as Ubuntu.**

First, add the Vigil Local APT repository (eg. for Debian `bookworm`):

```bash
echo "deb [signed-by=/usr/share/keyrings/valeriansaliou_vigil-local.gpg] https://packagecloud.io/valeriansaliou/vigil-local/debian/ bookworm main" > /etc/apt/sources.list.d/valeriansaliou_vigil-local.list
```

```bash
curl -fsSL https://packagecloud.io/valeriansaliou/vigil-local/gpgkey | gpg --dearmor -o /usr/share/keyrings/valeriansaliou_vigil-local.gpg
```

```bash
apt-get update
```

Then, install the Vigil Local package:

```bash
apt-get install vigil-local
```

Then, edit the pre-filled Vigil Local configuration file:

```bash
nano /etc/vigil-local.cfg
```

Finally, restart Vigil Local:

```
service vigil-local restart
```

**Install from Cargo:**

If you prefer managing `vigil-local` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install vigil-local
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Vigil Local using the `vigil-local` command.

**Install from source:**

The last option is to pull the source code from Git and compile Vigil Local via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

**Install from Docker Hub:**

You might find it convenient to run Vigil Local via Docker. You can find the pre-built Vigil Local image on Docker Hub as [valeriansaliou/vigil-local](https://hub.docker.com/r/valeriansaliou/vigil-local/).

First, pull the `valeriansaliou/vigil-local` image:

```bash
docker pull valeriansaliou/vigil-local:v1.2.2
```

Then, seed it a configuration file and run it (replace `/path/to/your/vigil-local/config.cfg` with the path to your configuration file):

```bash
docker run -v /path/to/your/vigil-local/config.cfg:/etc/vigil-local.cfg valeriansaliou/vigil-local:v1.2.2
```

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/vigil-local/blob/master/config.cfg) configuration file and adjust it to your own environment.

You can also use environment variables with string interpolation in your configuration file, eg. `token = ${VIGIL_TOKEN}`.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production

**[report]**

* `endpoint` (type: _string_, allowed: URL, no default) — Vigil status page master reporting URL (can be public via eg. HTTPS, or private over LAN; without trailing slash, eg. `https://status.example.com`)
* `token` (type: _string_, allowed: any string, no default) — Your master Vigil Reporter token (as configured in Vigil)

**[metrics]**

* `interval` (type: _integer_, allowed: seconds, default: `120`) — Interval for which to probe nodes in `poll` and `script` mode (ie. all nodes)
* `poll_retry` (type: _integer_, allowed: seconds, default: `2`) — Interval after which to try probe for a second time nodes in `poll` mode (only when the first check fails)
* `poll_delay_dead` (type: _integer_, allowed: seconds, default: `10`) — Delay after which a node in `poll` mode is to be considered `dead` (ie. check response delay)
* `poll_delay_sick` (type: _integer_, allowed: seconds, default: `1`) — Delay after which a node in `poll` mode is to be considered `sick` (ie. check response delay)

**[probe]**

**[[probe.service]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service

**[[probe.service.node]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service node
* `mode` (type: _string_, allowed: `poll`, `script`, no default) — Probe mode for this node (ie. `poll` is direct HTTP, TCP or ICMP poll to the URLs set in `replicas`, while `script` is used to execute a shell script)
* `replicas` (type: _array[string]_, allowed: TCP, ICMP or HTTP URLs, default: empty) — Node replica URLs to be probed (only used if `mode` is `poll`)
* `scripts` (type: _array[string]_, allowed: shell scripts as source code, default: empty) — Shell scripts to be executed on the system as a Vigil Local sub-process; they are handy to build custom probes (only used if `mode` is `script`)

### Run

Vigil Local can be run as such:

`./vigil-local -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Vigil Local, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Vigil Local daemon.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**
