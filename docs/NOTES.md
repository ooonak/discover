# Design thoughts

This project follows **Domain-Driven Design (DDD)** and **Hexagonal Architecture (Ports & Adapters)** patterns to separate business logic from infrastructure and user interfaces.

---

## Project Structure

```
.
├── application
│   ├── discover_service.rs
│   └── user_action_listener.rs
├── domain
│   ├── device_listener.rs
│   ├── device.rs
│   └── view_updates.rs
├── infrastructure
├── interface
│   └── avahi_device_listener.rs
└── main.rs
```

---

## Layer Descriptions

### domain — Core Business Logic

- Contains pure domain models and business rules, independent from UI or infrastructure.
- Examples: domain entities, domain ports (traits), and output ports for UI updates.

### application — Application Services

- Coordinates domain entities and handles use cases.
- Acts as a bridge between domain logic and external input.
- Contains ports for user actions and services for business workflows.

### interface — Adapters (Driving and Driven Sides)

- Implements adapters to external systems like UI, network, or APIs.
- Translates external events/messages into domain/application calls.

### infrastructure — Technical Implementations

- Technical details like databases, messaging, or external integrations.
- Supports interface adapters without containing business logic.

### main.rs — Composition Root

- Bootstraps the application by wiring dependencies.
- Instantiates domain, application, interface, and infrastructure components.

---

## Why Hexagonal Architecture and DDD?

- **Clear separation of concerns:** Domain logic is isolated from external details.
- **Inward dependencies:** Outer layers depend on domain/application abstractions, not the other way.
- **Testability:** Business logic can be tested without UI or infrastructure.
- **Flexibility:** Easier to swap or extend UI, network, or infrastructure independently.

---

## ASCII Hexagonal Diagram

```
            +----------------+
            |   Interface    |<------------------------+
            |  (Adapters)    |                         |
            +----------------+                         |
                   ^                                   |
                   |                                   |
          +--------+--------+                          |
          |                 |                          |
+---------+   Application   +------------------+       |
|         |   (Use Cases)   |                  |       |
|         +-----------------+                  |       |
|                  ^                          (User)   |
|                  |                                   |
|         +--------+--------+                          |
|         |                 |                          |
|         |     Domain      |                          |
|         | (Business Logic)|                          |
|         +-----------------+                          |
|                  ^                                   |
|                  |                                   |
|           +------+--------+                          |
|           | Infrastructure|                          |
|           | (Technical)   |                          |
|           +---------------+                          |
+------------------------------------------------------+
```

---

# Tooling

## D-BUS

Inspect by hand

```bash
$ gdbus introspect --system --dest org.freedesktop.Avahi --object-path / --recurse
```

### Introspection

No dynamic introspection here, Avahi interface is the same, static introspection.
```bash
# For humans
$ busctl introspect org.freedesktop.Avahi / org.freedesktop.Avahi.Server

# For code generation
$ busctl introspect --xml-interface org.freedesktop.Avahi / org.freedesktop.Avahi.Server > avahi-server-interface.xml
$ zbus-xmlgen file avahi-server-interface
```

I ended up downloading the official Avahi interface definitions from https://git.0pointer.net/avahi.git/tree/avahi-daemon/

```bash
$ zbus-xmlgen file docs/Server.xml 
$ zbus-xmlgen file docs/ServiceBrowserIntrospect.xml 
$ zbus-xmlgen file docs/ServiceResolverIntrospect.xml 

# Or just. The same object path contains two interfaces, Server2 is a newer, extended version of Server.
$ zbus-xmlgen system org.freedesktop.Avahi /
Skipping `org.freedesktop.DBus` interfaces, please use https://docs.rs/zbus/latest/zbus/fdo/index.html
Generated code for `org.freedesktop.Avahi.Server` in server.rs
Generated code for `org.freedesktop.Avahi.Server2` in server2.rs
```
