# esp-webrtc-sys

Low-level **`#![no_std]`** FFI bindings to Espressif's [`esp-webrtc-solution`](https://github.com/espressif/esp-webrtc-solution): the `esp_webrtc` peer-connection orchestrator, the `esp_peer` ICE/DTLS/SRTP agent (`peer_default`), and WHIP signaling.

This crate does **not** call into the C libraries by itself — it only declares types and `extern "C"` symbols. Native code is linked when your firmware is built as an **esp-idf** project and the components below are registered with the linker (see [Linking](#linking)). The safe wrapper consuming this crate is [`esp-webrtc`](../esp-webrtc), a sibling path dependency; application code should depend on that, not on this crate directly.

## Requirements

- A Rust **ESP-IDF** application (e.g. [`esp-idf-sys`](https://github.com/esp-rs/esp-idf-sys) + [`esp-idf-svc`](https://github.com/esp-rs/esp-idf-svc)) that pulls this crate in as a dependency.
- The **parent** crate must be the one that embeds ESP-IDF so [`esp-idf-sys` can read](https://doc.rust-lang.org/cargo/reference/metadata.html) `[package.metadata.esp-idf-sys]` from a path dependency (typical pattern: your app depends on `esp-webrtc` → `esp-webrtc-sys`, both by `path = "..."`).
- Both `esp-webrtc` and `esp-webrtc-sys` must be **path dependencies**, not git dependencies, resolving to the same source. Otherwise Cargo builds two copies and the FFI types passed across the safe wrapper's boundary do not unify.

## Linking

`Cargo.toml` registers native components via `extra_components`:

| Component (under `esp-webrtc-solution/components/`) | Provides |
|---|---|
| `esp_webrtc` | Peer-connection orchestrator: `esp_webrtc_open`/`start`/`stop`/`close`, event handling, WHIP signaling glue |
| `esp_peer` | The `peer_default` ICE/DTLS/SRTP agent — prebuilt per-chip archive, see [`esp_peer`](#esp_peer-the-prebuilt-ice-agent) below |
| `webrtc_utils` | Shared media/RTP utilities `esp_webrtc` and `esp_peer` depend on |
| `media_lib_utils` | Thread/timer/event-group abstraction the peer's control-plane threads (`pc_task`, `pc_send`) run on |
| `av_render` | Audio/video render pipeline (unused by this project's audio-only path, but required to link) |
| `codec_board` | Board-level codec/capture glue required by `esp_webrtc`'s build |

Paths are relative to this crate's manifest (`esp-webrtc-sys/Cargo.toml`), so they resolve to the `esp-webrtc-solution` submodule checked out alongside it (see [Fetching `esp-webrtc-solution`](#fetching-esp-webrtc-solution) below).

## Fetching `esp-webrtc-solution`

`esp-webrtc-solution` is a **git submodule** of this repo, checked out at `esp-webrtc-sys/esp-webrtc-solution`. Populate it with:

```sh
git submodule update --init esp-webrtc-solution
```

`.gitmodules` points it at **`https://github.com/vinhlq/esp-webrtc-solution.git`** — a fork, not upstream `espressif/esp-webrtc-solution` — because the component manifests need `override_path` entries so `esp_webrtc`'s dependencies (`av_render`, `media_lib_utils`) resolve to the sibling directories actually vendored here instead of trying to fetch them from the ESP Component Registry. That fix is `esp-webrtc-solution.patch` at this repo's root, already applied and committed in the fork (see the fork's own commit history) — the `.patch` file is kept only as a readable record of what changed, `build.rs` does not apply it.

## `esp_peer`: the prebuilt ICE agent

`esp_peer`'s `peer_default` implementation ships as a **prebuilt static archive per chip** (`components/esp_peer/libs/<chip>/libpeer_default.a`), currently pinned at v1.5.1 — there is no source to read for it, only the vendored headers (`esp_peer.h`, `esp_peer_types.h`, `esp_peer_default.h`) and the archive's own symbol table. Two things follow from that:

- **Nothing here catches a header/library mismatch except the layout assertions below.** If you bump `esp_peer`'s version, re-verify every struct this crate hand-transcribes against the new headers.
- **Undocumented behavior has to be found by disassembly, not by reading source.** For example, `agent_set_tcp_only` (declared in this crate, see its doc comment) has no header at all — its signature and the fields it actually gates were confirmed from the archive's DWARF debug info and from tracing which functions read the flag it writes, not from any public API surface.

## Hand-transcribed bindings, not bindgen

Despite `bindgen` being a build-dependency, **`build.rs` does not run it** — every type in `src/lib.rs` is transcribed by hand from the vendored headers, and correctness is enforced by a block of `const` layout assertions at the bottom of the file (sizes, alignments, and field offsets for every `#[repr(C)]` struct, derived by compiling the real headers for the target with `xtensa-esp32s3-elf-gcc` and cross-checked with host `gcc -m32`). A vendor update that reorders a field fails the build with a specific offset mismatch instead of silently miscompiling — a mistake here reads a callback pointer out of what Rust wrote as an integer, and looks like a crash somewhere unrelated. When adding a new binding, add its layout assertions in the same commit; when bumping `esp-webrtc-solution`, the assertions failing is the signal to re-check every offset, not just the ones the compiler complains about.

`wrapper.h` exists for a future bindgen-based regeneration and is not currently referenced by any build step.

## Safety

All FFI entry points are **`unsafe`**. You must uphold C API contracts (valid pointers, lifetimes, init order, thread context). Prefer the safe wrapper in [`esp-webrtc`](../esp-webrtc) over calling into this crate directly.

## License

Licensed under **MIT OR Apache-2.0**, matching common `esp-rs` crates (see `Cargo.toml`).
