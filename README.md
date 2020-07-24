Vigil Local
===========

[![Build Status](https://travis-ci.org/valeriansaliou/vigil-local.svg?branch=master)](https://travis-ci.org/valeriansaliou/vigil-local)

**Vigil Local is used to monitor hosts behind a firewall and report their status to Vigil.**

Vigil Local is an (optional) daemon that you can use to report internal service health to your [Vigil-powered status page](https://github.com/valeriansaliou/vigil). It is designed to be used behind a firewall, and to monitor hosts bound to a local loop or LAN network, that are not available to your main Vigil status page. It can prove useful as well if you want to fully isolate your Vigil status page from your internal services.

Install Vigil Local on a server of yours and configure it with your [Vigil](https://github.com/valeriansaliou/vigil) endpoint URL and token; it will then start monitoring all configured nodes and report them to Vigil. Make sure that you pre-configure all local nodes as `local` in Vigil, and then as `poll` or `script` in Vigil Local, accordingly.

_Tested at Rust version: `rustc 1.44.1 (c7087fe00 2020-06-17)`_

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

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production

**[report]**

* `endpoint` (type: _string_, allowed: URL, no default) — Vigil status page reporting URL (can be public via eg. HTTPS, or private over LAN; without trailing slash, eg. `https://status.example.com`)
* `token` (type: _string_, allowed: any string, no default) — Your Vigil Reporter token (as configured in Vigil)

**[metrics]**

* `poll_interval` (type: _integer_, allowed: seconds, default: `60`) — Interval for which to probe nodes in `poll` mode
* `poll_retry` (type: _integer_, allowed: seconds, default: `2`) — Interval after which to try probe for a second time nodes in `poll` mode (only when the first check fails)
* `poll_http_status_healthy_above` (type: _integer_, allowed: HTTP status code, default: `200`) — HTTP status above which `poll` checks to HTTP replicas reports as `healthy`
* `poll_http_status_healthy_below` (type: _integer_, allowed: HTTP status code, default: `400`) — HTTP status under which `poll` checks to HTTP replicas reports as `healthy`
* `poll_delay_dead` (type: _integer_, allowed: seconds, default: `10`) — Delay after which a node in `poll` mode is to be considered `dead` (ie. check response delay)
* `poll_delay_sick` (type: _integer_, allowed: seconds, default: `1`) — Delay after which a node in `poll` mode is to be considered `sick` (ie. check response delay)
* `script_interval` (type: _integer_, allowed: seconds, default: `180`) — Interval for which to probe nodes in `script` mode

**[probe]**

**[[probe.service]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service

**[[probe.service.node]]**

* `id` (type: _string_, allowed: any unique lowercase string, no default) — Unique identifier of the probed service node
* `mode` (type: _string_, allowed: `poll`, `script`, no default) — Probe mode for this node (ie. `poll` is direct HTTP, TCP or ICMP poll to the URLs set in `replicas`, while `script` is used to execute a shell script)
* `replicas` (type: _array[string]_, allowed: TCP, ICMP or HTTP URLs, default: empty) — Node replica URLs to be probed (only used if `mode` is `poll`)
* `scripts` (type: _array[string]_, allowed: shell scripts as source code, default: empty) — Shell scripts to be executed on the system as a Vigil Local sub-process; they are handy to build custom probes (only used if `mode` is `script`)
* `http_body_healthy_match` (type: _string_, allowed: regular expressions, no default) — HTTP response body for which to report node replica as `healthy` (if the body does not match, the replica will be reported as `dead`, even if the status code check passes; the check uses a `GET` rather than the usual `HEAD` if this option is set)

### Run

Vigil Local can be run as such:

`./vigil-local -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Vigil Local, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Vigil Local daemon.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**

**:gift: Based on the severity of the vulnerability, I may offer a $100 (US) bounty to whomever reported it.**
