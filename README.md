# 🦆 Paddler

Paddler is a simple, extensible, word-level pattern checker.

Paddler is **not** a pattern extractor, nor is it a replacement for, update over, or an alternative to regex.

Paddler returns matching words that match the given pattern, given a list of words or a file, separating words by newline.

## Syntax

Paddler has a couple simple tokens that can be found in [engine.rs](./src/engine.rs#L13):

```rust
pub enum Token {
    Group(char),
    Boundary(Boundary),
    Char(char),
}
```

### Tokens

- **Boundaries:**
  - `^` : Word start
  - `$` : Word ending
  - `.` : Syllable boundary
- **Groups:** An uppercase character (e.g., `A`, `B`).
- **Chars:** Any lowercase alphabetical character (e.g., `a`, `b`).

## Usage

```bash
paddler [OPTIONS] --pattern <PATTERN> <--file <FILE>|INPUT>
```

### Examples

```bash
$ paddler --pattern elmV elma
elma
```

Paddler can process more than one word at a time:

```bash
$ paddler --pattern elmV elma elmas adak
elma
elmas
```

Paddler can be used to traverse over a file of words, separated via newlines:

```bash
$ paddler --pattern ^CV.CV$ --file words.txt
masa
kasa
...
```

This will output all the words that match to the given pattern from words.txt.

## Groups

Paddler has a concept of `groups`.

A group is a set of characters, for example a group named `X` could be equal to: `["a", "b", "c"]`. Now wherever you use `X` in your pattern it will match any of the given characters `a, b, c`.

Groups are defined by *uppercase single chars*. By default paddler comes shipped with two groups defined: `C` for consonants, `V` for vowels, of Turkish.

With the `-c, --config` flag you can choose your own config file.

## Config

A default [config file](config.json) is included in the repo along with the [config schema](config.schema.json).
