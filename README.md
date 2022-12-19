# collate-rs: a Rust Implementation of Unicode Collation Algorithm

**Beware:** This is a work in progress intended as a way to learn Rust and the UCA.
The contents of this document as well as the source code of this project are mostly
for personal use in the current state.

## Introduction

This project is an implementation of the Unicode Collation Algorithm (UCA) in Rust,
as specified in the [Unicode Technical Standard 10](https:://www.unicode.org/reports/tr10).
In its current state, it is a learning project for myself, both on UCA and Rust.

The final goal is to archive conformance as defined in [UTS10](https://www.unicode.org/reports/tr10/#Conformance),
and provide an API for deriving collation tables _on demand_.

**Current state:** Basic implementation of part of the [definitions](https:://www.unicode.org/reports/tr10/#Definitions)

## Motivations

- I enjoy programming in Rust.
- When trying to sort strings on alphabetical order, the strings I'm working with
  often come from texts in Ancient Indo-European languages. This means that
  frequently there are no _out of the box_ means to sort them
  according to the criteria used by their speakers or the modern scholars
  working with them.
- Technical Standards are weird grammars when you are a linguist, so why not?
- `unic_coll` is no longer available, so in the case I actually finish this project,
  I might contribute (and this unprofessional text might disappear from internet,
  replaced by something useful).

## Todo's

1. [ ] Implement the definitions defined in [UTS10§3](https://www.unicode.org/reports/tr10/#Definitions)
       ~that do not require other parts of the algorithm to work~.
2. [ ] Implement the sections §4-12 of [UTS10](https://www.unicode.org/reports/tr10/#Variable_Weighting)
       and the definitions lacking from 1.
3. [ ] Pass the conformance tests defined in [UTS10§2](https://www.unicode.org/reports/tr10/#Conformance).
4. [ ] Provide an API for generating collation tables _on demand_, so that I can
       happily sort text in Anatolian Hieroglyphs or accentuated Vedic in Devanagari
       or IAST or HK as I see fit.
5. [ ] Fork it and optimize it if it comes to the point of being useful for non-niche
       purposes.

## Project Structure

As this is a project intended for me to learn UCA, I will force myself
(and any hypothetical contributors) to follow some constraints while structuring
this project.

**Before anything:** _docstring_ the hell out of it.

1. Every section §x in the [UTS10](https://www.unicode.org/reports/tr10/) must be
   its own module. The sole exception is the [UTS10§1 Introduction](https://www.unicode.org/reports/tr10/#Introduction),
   but for now I will keep the file `src/introduction/mod.rs`.
2. If subsections define different processes, types, etc, the `src/section/mod.rs`
   will define a module for each relevant subsection, to be stored in a file `src/section/subsection.rs`.
3. The definitions from [UTS10§3](https://www.unicode.org/reports/tr10/#Definitions)
   will be overtly included in the _docstrings_ of their implementation.
   In the case of the other sections, they may follow the same criteria if relevant.
   Example from `src/definitions/collocation_wel.rs`:

   ```rust
   /// > UTS10-D1. Collation Weight: A non-negative integer used in the UCA to
   /// > establish a means for systematic comparison of constructed sort keys.
   #[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
   pub struct CollationWeight(pub(crate) u32);
   ```

   a. Traits or methods that satisfy a definition from subsection §x implemented
   for a `struct` or a `enum` defined in section §y, must pertain to the source
   code for section §x and not §y. This might be suboptimal from a project structure
   point of view, but will keep the order of implementations for definitions
   as close as possible from the [UTS10](https://www.unicode.org/reports/tr10/)
   structure.

4. If something does not need to be public, but the documentation is important for
   learning purposes, make it public, this is a literate programming project, maybe.
