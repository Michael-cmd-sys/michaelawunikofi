# Tailwind Styling Migration Design

## Summary

This change will consolidate the portfolio site onto a single Tailwind-driven styling system while preserving the current visual identity. The migration will remove the remaining dependency on `assets/main.css`, keep the current warm editorial palette and serif/sans pairing, and fix the cramped feel in project cards, technology pills, and article-style text blocks.

## Goals

- Port the remaining handcrafted CSS rules into Tailwind-native styling.
- Preserve the existing aesthetic rather than redesigning the site.
- Improve spacing and readability in cards, pills, and long-form paragraphs.
- Reduce duplicate or stale styling rules that no longer match the current RSX.

## Non-Goals

- No visual rebrand, palette shift, or typography overhaul.
- No structural content rewrite.
- No routing, data, or component behavior changes.

## Current State

The site already uses Tailwind utility classes extensively in `src/main.rs`, but it still loads `assets/main.css` for:

- global font imports and body defaults
- section shell spacing and background treatments
- article copy spacing (`about-lead`, `about-body`)
- footer styling
- legacy navbar and hero selectors that duplicate or conflict with current RSX-driven styling
- custom keyframes and animation helpers

This creates a mixed styling model with duplicated responsibility between utility classes and legacy CSS.

## Recommended Approach

Use `tailwind.css` as the single styling source of truth.

Implementation shape:

- Keep `assets/tailwind.css` as the generated output stylesheet.
- Expand the root `tailwind.css` source with Tailwind-native theme tokens and custom utilities/components.
- Remove `document::Stylesheet { href: MAIN_CSS }` from `src/main.rs`.
- Replace semantic legacy classes in RSX with either:
  - direct Tailwind utilities where the usage is local and one-off
  - small Tailwind component-layer classes where the same pattern repeats

This keeps the styling system coherent without turning every component into an unreadable wall of utility classes.

## Styling Architecture

### Theme Layer

The Tailwind source file will own the existing design tokens:

- warm background colors
- muted body copy colors
- accent colors used for labels, pills, and links
- shadow scale
- rounded corner scale
- animation names and timing helpers
- font family aliases for body and heading usage

The token values should closely match the current site so the visual identity stays stable.

### Component Layer

Repeated patterns should move into Tailwind component classes inside `tailwind.css`:

- section shells with consistent horizontal padding and vertical rhythm
- highlighted sections like About and Contact
- project article cards
- technology/status pills
- article-copy paragraph styles
- footer text styles
- shared animation delay helpers defined in `tailwind.css` so the current staggered motion remains explicit and discoverable

These classes should stay shallow and presentational, not become a second bespoke CSS system.

### RSX Layer

`src/main.rs` remains the place where layout composition is obvious:

- section structure
- heading hierarchy
- content grouping
- one-off responsive layout classes

The RSX should keep using utilities for local layout decisions and rely on component classes only for recurring visual patterns.

## Spacing and Readability Adjustments

### Cards

Project cards should get more internal breathing room:

- slightly larger padding on all breakpoints
- more space between title/status, description, tech stack, and optional link
- improved separation between stacked content blocks

The card silhouette, colors, borders, and hover motion should remain effectively the same.

### Pills

Status tags and technology pills should feel less cramped:

- increase horizontal padding
- slightly increase vertical padding
- allow a bit more wrap gap between adjacent pills
- preserve existing rounded pill styling and color semantics

### Articles and Long-Form Copy

About and contact copy should read more comfortably:

- increase paragraph spacing
- slightly loosen line height
- ensure the lead paragraph has stronger separation from supporting paragraphs

This applies to text blocks that currently use the legacy `about-lead` and `about-body` classes.

## File-Level Plan

### `src/main.rs`

- remove the `MAIN_CSS` asset reference and stylesheet injection
- replace remaining legacy classes such as `about-section`, `about-lead`, `about-body`, `projects-section`, `techstack-section`, `contact-section`, `footer`, and `footer-tech`
- keep the current structure and content intact
- only adjust class lists where spacing and Tailwind migration require it

### `tailwind.css`

- keep the existing Tailwind imports and source glob
- add theme tokens, shared component classes, and any custom animation helpers
- mirror the current visual values closely enough that the change reads as a cleanup, not a redesign

### `assets/main.css`

- remove the file after migration
- do not leave compatibility rules behind in a second stylesheet

## Responsiveness

Spacing changes must hold on both mobile and desktop:

- card padding should stay generous without causing crowded wrapping
- pill rows should wrap cleanly on narrow widths
- article text blocks should keep comfortable margins without becoming too wide
- highlighted sections should maintain their inset panel look without pinching the content area

## Risks and Mitigations

### Risk: Tailwind output misses dynamically constructed classes

Mitigation:

- avoid generating arbitrary class names at runtime where possible
- keep animation delay helpers explicit
- verify the final build includes any custom classes added in `tailwind.css`

### Risk: visual drift during migration

Mitigation:

- preserve existing token values first, then adjust only spacing
- avoid changing color semantics, typography hierarchy, or section order

### Risk: removing `assets/main.css` drops global defaults

Mitigation:

- move fonts, body colors, and scroll behavior into Tailwind-managed layers before removing the stylesheet reference

## Testing and Verification

- run a local build/check to confirm the Rust app still compiles
- verify the generated Tailwind stylesheet still builds correctly
- visually inspect the homepage sections with emphasis on:
  - project cards
  - status and tech pills
  - about/contact article copy
  - footer styling
- confirm no stale references to `assets/main.css` remain

## Acceptance Criteria

- the app uses Tailwind as its only styling system for the current page
- `assets/main.css` is no longer required
- the page looks materially the same except for improved spacing and readability
- cards, pills, and article copy have noticeably better breathing room
- no obvious regressions in responsive layout or hover/animation behavior
