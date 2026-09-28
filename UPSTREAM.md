# Upstream tracking

This fork (`oer/main` at `github.com/ermacv/openthread`) follows
`github.com/esp-rs/openthread` `main` by periodic merges into `oer/main`,
every one to two weeks. It is never rebased: every pinned revision stays
reachable and carries a `pin/<sha>` tag, and each merge records what it took.
`main` in this repository mirrors upstream unchanged. Former branches are kept
as `archive/<branch>` tags.

## Last merge

- Upstream: `b01e7e63a48943d060e971e7ea364a84609c7ffb` (2026-09-28,
  "Update esp-radio to beta.1 (#120)").
- Previous merge base: `c8ba3f307d0bc4384fd88ee17d51cd51736fb6ab`
  ("New release (#118)").

To prepare the next merge, `git fetch upstream` and review
`git log <last merged upstream>..upstream/main`.

## Why the fork differs

The open-esp-radio-rs IEEE 802.15.4 radio drives OpenThread through the
`Radio` trait with radio features upstream does not expose:

- **Transmit security.** A radio that secures frames itself receives the key
  material and frame counter, as `OT_RADIO_CAPS_TRANSMIT_SEC` requires.
- **Coordinated Sampled Listening.** CSL period, channel and sample time reach
  the radio, and the radio writes the CSL IE phase.
- **Enhanced-ACK probing.** A Link Metrics subject configures the radio's
  enhanced-ACK probing.
- **Thread network time synchronization.** The radio fills the Time IE.
- **Live RSSI.** `otPlatRadioGetRssi` reads the radio live.
- **C-free `openthread-radio` crate.** The `Radio` trait and its value types
  live in a crate without the OpenThread C build, so radio implementations
  compile and test without it.

## Upstream changes not taken

None.
