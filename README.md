Vigil Local
===========

[![Build Status](https://travis-ci.org/valeriansaliou/vigil-local.svg?branch=master)](https://travis-ci.org/valeriansaliou/vigil-local)

**Vigil Local is used to monitor internal hosts and report their status to Vigil.**

Vigil Local is a daemon that you can use to report internal service health to your Vigil-powered status page. It is designed to be used behind a firewall, and to monitor hosts bound to a local loop or LAN network that is not available to your main Vigil status page. It can prove useful as well if you want to fully isolate your Vigil status page from your internal services.

Install Vigil Local on a server of yours and configure it with your Vigil endpoint URL and token; it will then automatically start monitoring all configured nodes and report them to Vigil. Make sure that you pre-configure all local nodes as `local` in Vigil, and then as `poll` or `script` in Vigil Local, accordingly.

_Tested at Rust version: `rustc 1.44.1 (c7087fe00 2020-06-17)`_

**🇧🇬 Crafted in Sofia, Bulgaria.**

## What is Vigil?

Vigil is an open-source Status Page you can host on your infrastructure, used to monitor all your servers and apps, and visible to your users.

It lets you monitor your critical systems using a variety of methods: `push` for applications using a Vigil Reporter library, `poll` for Vigil-reachable HTTP, TCP & ICMP services, `script` to execute custom probes, and `local` for non-Vigil-reachable private services.

Vigil Local lets you monitor nodes that are configured in `local` mode, aside other monitoring methods that do not require the Vigil Local utility.

## How to use it?

### Installation

**Install from binary:**

A pre-built binary of Vigil Local is shared in the releases on GitHub. You can simply download the latest binary version from the [releases page](https://github.com/valeriansaliou/vigil-local/releases), and run it on your server.

You will still need to provide the binary with the configuration file, so make sure you have a Vigil Local `config.cfg` file ready somewhere.

_The binary provided is statically-linked, which means that it will be able to run on any Linux-based server. Still, it will not work on MacOS or Windows machines._

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
docker pull valeriansaliou/vigil-local:v1.0.0
```

Then, seed it a configuration file and run it (replace `/path/to/your/vigil-local/config.cfg` with the path to your configuration file):

```bash
docker run -v /path/to/your/vigil-local/config.cfg:/etc/vigil-local.cfg valeriansaliou/vigil-local:v1.0.0
```

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/vigil-local/blob/master/config.cfg) configuration file and adjust it to your own environment.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `warn`) — Verbosity of logging, set it to `error` in production

**[report]**

* `endpoint` (type: _string_, allowed: URL, no default) — Vigil status page reporting URL (can be public via eg. HTTPS, or private over LAN; without trailing slash, eg. `https://status.example.com`)
* `token` (type: _string_, allowed: any string, no default) — Your Vigil Reporter token (as configured in Vigil)

### Run

Vigil Local can be run as such:

`./vigil-local -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Vigil Local, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Vigil Local daemon.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**

**:gift: Based on the severity of the vulnerability, I may offer a $100 (US) bounty to whomever reported it.**
