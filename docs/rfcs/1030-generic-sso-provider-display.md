<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# [RFC] Generic SSO provider display in the GUI

## Overview

Today the GUI only knows how to display a hardcoded, closed list of SSO providers. Adding a new
provider requires a GUI code change and a new release, which is highly impractical and doesn't let
us handle private-SSOs.

This RFC proposes that the GUI stop hardcoding anything about specific SSO providers. Instead, it
consumes a list of self-describing provider objects (name, an info link, and an icon, on top of
the existing technical mountpoint) and renders every one of them through a single generic button
component. The GUI becomes provider-agnostic: any SSO configured server-side, well-known or fully
custom, shows up automatically with a consistent, presentable look.

This RFC only covers the **GUI** side of the change **for now** (feel free to add to it), and
leaves out how the server and libparsec serve that list. It starts from the assumption that:

> The GUI gets a list of complex objects from libparsec.

## Background & Motivation

Client-side, the list of SSO is a closed set:

- `OpenBaoAuthConfigTag` is a fixed enum (`OIDCHexagone`, `OIDCProConnect`, ...). Every new
  provider requires adding a new variant to this enum, which means a libparsec change.
- `client/src/services/openBao.ts` keeps a `HandledSSOProviders` allow-list and an
  `isSSOProviderHandled()` guard. Any tag not in that list is silently dropped, even if the server
  advertises it.
- `client/src/components/devices/SsoProviderCard.vue` only ever renders a branch for
  `OpenBaoAuthConfigTag.OIDCProConnect`. That branch hardcodes the ProConnect brand assets
  (`pro-connect1.svg` / `pro-connect2.svg`) and hardcoded i18n keys (`proConnect.title`,
  `proConnect.description`, `proConnect.link`).
- `client/src/components/devices/ConnectSso.vue` filters `serverConfig.openbao.auths` through
  `isSSOProviderHandled` before rendering one `SsoProviderCard` per remaining entry.

In short: as soon as a Parsec server operator enables SSO with a provider Parsec doesn't already
know about by name (their own OIDC IdP, an organization-specific Hexagone/ProConnect-like
connector, etc.), the GUI just doesn't show a button for it, no matter what the server advertises.
There is no way for a customer to bring their own SSO today without us shipping a new GUI build
with new hardcoded branding for them.

## Goals and Non-Goals

**Goals**

- Let the GUI display an arbitrary number of SSO providers, including providers Parsec has never
  heard of, without any GUI code change.
- Give server operators a way to brand their own SSO entry (a display name, an icon, a link to
  documentation/help) so the button isn't a bare unlabeled placeholder.
- Keep the resulting list of buttons visually consistent and "presentable" even though the
  underlying providers are arbitrary and uncontrolled.
- Keep the existing authentication flow (`openBaoConnect`, popup/redirect handling, connected-state
  display) unchanged - only the *discovery and display* of providers changes.

## Design

### 1. Data the GUI expects to receive

This RFC assumes the GUI receives, for each configured SSO provider, an object shaped roughly
like:

```ts
interface SsoProviderDisplay {
  id: string; // opaque identifier, passed back unchanged to select this provider
  name: string; // display name, e.g. "ProConnect", "Acme Corp SSO"
  mountPath: string; // currently implemented
  link?: string; // URL to documentation/help about this provider, if any
  icon?: string; // something the GUI can render as an image (see 4.)
}
```

The exact wire format (in particular whether `name` is a plain string or needs to support multiple
languages, and the concrete encoding of `icon`) is for the libparsec RFC to define. This RFC only
requires that the GUI can render an image and a string, and can round-trip an opaque identifier
back into the existing `openBaoConnect(..., provider, ...)` call in place of today's
`OpenBaoAuthConfigTag`.

### 2. A single generic provider button

`SsoProviderCard.vue` stops branching on `provider === OpenBaoAuthConfigTag.*` and becomes fully
generic: it takes a `SsoProviderDisplay` (plus the existing `isConnected` prop) and always renders
the same structure:

- a clickable button containing the provider `icon` and `name`,
- an optional secondary line with a link to `link`, to provide the user with some documentation on
  the SSO.
- the existing "connected" state (checkmark + "Authentication.method.sso.connected" text), reused
  unchanged since it doesn't depend on the provider.

Because the button is now generic, its chrome (border, padding, radius, hover state) is owned by
Parsec's design system rather than by a provider-supplied image, and the icon/name are laid out
*inside* that shared chrome. This is a deliberate change from today's ProConnect card, whose whole
clickable surface *is* a brand-supplied bitmap. It keeps a list of many, visually unrelated
providers from looking like a pile of random buttons of different sizes and styles.

Sensible fallbacks are needed since data comes from an arbitrary server config:

- no `icon` -> a generic placeholder icon (e.g. a lock/key `ion-icon`) shipped with the GUI,
- no `link` -> the secondary line is simply not rendered,
- missing/empty id/name/mountPath -> the button is not displayed.

### 3. `ConnectSso.vue` becomes provider-agnostic

`ConnectSso.vue` drops the `isSSOProviderHandled` filtering entirely: it renders one generic
`SsoProviderCard` per entry libparsec returns, in the order given. The `sso-selected` event now
carries the provider's `id` instead of an `OpenBaoAuthConfigTag`, and is passed straight through to
`openBaoConnect`.

`client/src/services/openBao.ts` loses `HandledSSOProviders` and `isSSOProviderHandled()` - there
is no more "known vs. unknown provider" distinction on the GUI side, every provider the server
advertises is displayable.

### 4. Icon delivery and rendering constraints

Whatever format libparsec ends up choosing for `icon` (embedded data URI vs. a URL served by the
Parsec server vs. something else), the GUI side should hold to a few constraints regardless:

- The icon is only ever rendered as an `<img>` (or an equivalent CSS `background-image`) bound to
  that value. It is never interpreted as raw SVG/HTML markup (no `v-html`), to avoid turning an
  arbitrary server-controlled string into script execution.
- Loading the icon must be best-effort: a missing, oversized, malformed, or slow-to-load icon falls
  back to the generic placeholder and never blocks or hides the button itself (the provider must
  stay clickable even if its icon fails).
- The icon is displayed inside a fixed-size slot (e.g. a square of a fixed number of pixels,
  `object-fit: contain`), so a provider can't submit an oversized or oddly-shaped image and break
  the list's layout.
- If the icon turns out to be a remotely-hosted URL rather than something embedded in the config
  payload, that's an extra network request the GUI makes on the operator's behalf every time the
  login screen is shown; this has minor tracking/availability implications worth flagging to
  whoever designs the libparsec side (an embedded data URI avoids both, at the cost of a larger
  config payload).

### 5. Multi-language support for `name` / `link`

Unlike today's `proConnect.title` / `proConnect.description`, which go through the GUI's own i18n
catalog, a generic `name`/`link` supplied by the server config has no notion of the user's locale
unless the future libparsec/server API explicitly carries multiple translations. For this RFC, the
GUI simply displays `name` verbatim, in whatever language the operator configured it in. If
localization turns out to matter, it needs to be solved server-side (e.g. a `Translatable`-like
structure), not by the GUI guessing a locale.

## Alternatives Considered

- **Status quo, extended per-provider**: keep hardcoding one Vue branch and one set of brand assets
  per provider in the GUI, and add new ones as they come up. Doesn't scale (a GUI release is
  required per customer's custom IdP) and is the exact problem this RFC solves.

- **Display iframes served by the server**: server serves HTML page containing a button, shown in
  the GUI with `<iframe src=`. While this solve the generic look the button can have, it introduces
  new problems, most notably:
  - lots of HTTP requests
  - doesn't solve the translation issues and can cause problem with the theme
  - provide an attacker with a potential phishing opportunity (replace the button with a fake form
    asking for a password for example)

- **Hybrid: keep a small hardcoded "known good" style map, generic fallback for the rest**: keep a
  lookup table (keyed by the provider's `id`) of pixel-perfect brand assets for a handful of
  providers Parsec explicitly supports (e.g. ProConnect), and fall back to the generic renderer for
  everything else. This preserves brand-perfect buttons for well-known providers while still
  supporting arbitrary custom ones. Viable middle ground; see the ProConnect discussion in
  [Risks](#risks) for why this RFC doesn't pick it outright.

- **Closed icon set instead of arbitrary images**: instead of letting the server supply an actual
  image, the GUI ships a small fixed library of icons (e.g. "generic-oidc", "google", "microsoft")
  and the server config just picks one by name. Simpler trust model (no arbitrary image rendering
  at all) but defeats the "bring your own branding" goal for a company's own SSO, and still
  requires a GUI release to add a new icon to the library.

## Operations

GUI-only change, no new regular process for the Parsec team. It does shift a small amount of
responsibility to server operators/administrators: they now directly control what name, link and
icon show up in the Parsec client for their SSO entries, and a badly configured one (broken link,
missing icon, confusing name) is no longer something a Parsec GUI release can silently paper over.

## Security/Privacy/Compliance

- **Arbitrary link**: clicking a provider opens `link` in the system browser via the existing
  `Env.Links.openUrl()` helper (never an in-app webview), which is already how `proConnect.link`
  behaves today. This doesn't introduce a new mechanism, but it does widen what can be linked from
  inside the Parsec client to whatever an org's server config sets - the server config is already a
  trust boundary the client relies on today (it already dictates the OpenBao server URL, mount
  paths, etc.), so this isn't a new trust boundary, just a broader use of an existing one.
- **Arbitrary icon**: must strictly be rendered as an image resource, never as raw markup, to
  prevent an SSO entry's icon field from becoming an XSS vector (see 4. above).
- **Social-engineering surface**: because `name`/`link` are free text controlled by whoever
  configures the server, a compromised or malicious org admin could label an entry misleadingly
  (e.g. a `name` designed to look like "Reset your password" pointing to a credential-harvesting
  page). This is not fundamentally new (the same admin already controls the OpenBao server URL used
  for real authentication), but the attack becomes slightly cheaper/more visible since it's now a
  UI element instead of a URL, so this is worth flagging to the security team when the
  libparsec/server side of this feature is designed.

## Risks

- **ProConnect visual regression**: ProConnect currently gets a bespoke, pixel-perfect button image
  mandated by the French state design system, not a generic "icon + name" pill. Moving it through
  the generic renderer means it loses that exact look. This RFC leans towards accepting the generic
  look for all providers (including ProConnect) for the sake of consistency and simplicity, but
  this is a product/design call, and ProConnect's branding requirements may be a compliance matter
  worth double-checking before committing to it (see [Alternatives](#alternatives-considered) for
  the hybrid fallback option).
- **Layout breakage from bad input**: since icon/name/link now come from an uncontrolled source,
  the GUI must defensively clamp icon size and handle missing/failed data (see 4.); skipping this
  could make the SSO list ugly or broken on a misconfigured server.
- **Design churn once libparsec lands**: this RFC works from an assumed shape for
  `SsoProviderDisplay` since the libparsec/server side isn't designed yet. The actual GUI
  implementation may need adjusting once that follow-up RFC settles the real data contract
  (especially icon encoding and whether `name`/`link` support localization).

## Remarks & open questions

- What identifier does the GUI pass back to select a provider once `OpenBaoAuthConfigTag` is no
  longer a closed enum? This RFC assumes an opaque `id` round-tripped from the list entry, but the
  concrete type is for the libparsec RFC to define.
- Does ProConnect keep a dedicated, brand-compliant button, or move fully to the generic renderer
  like every other provider (see Risks)?
- Is the order libparsec returns providers in meaningful, or should the GUI impose its own ordering
  (e.g. alphabetical by `name`)?
- Should provider `name`/`link` support localization, and if so, whose responsibility is it (server
  config vs. GUI)?
- What are the concrete size/format constraints the GUI should document and enforce for `icon`
  (max dimensions, allowed formats, data URI vs. URL)?
