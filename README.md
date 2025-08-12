# Discover

## Usage

For a quick spin, one can just advertise a fake service like this.

```
$ avahi-publish-service "TestDevice" _discover._tcp 0 hw=deadbeef sn=12345678
```

And then run the application from another console.

```
$ RUST_LOG=info cargo run
```

## Overview

Discover is a TUI application used to discover devices. It's a Linux application that uses the Avahi protocol via D-BUS.

Avahi implements the Apple Zeroconf specification, mDNS, DNS-SD and RFC 3927/IPv4LL. Using multicast DNS and DNS Service Discovery Avahi provides zero configuration service discovery.

## Background

I needed a project to improve my skills and knowledge around Rust, hexagonal architecture (ports & adapters) and DDD for embedded applications.
