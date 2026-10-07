# 🦆 Paddler

Paddler is a simple, extensible, word-level phonetic pattern checker.

Paddler is **not** a pattern extractor, nor is it a replacement for, update over, or an alternative to regex.

Paddler returns words that match the given phonetic pattern, given a dictionary file, separating words by newline.

**License:** This project is licensed under the LGPL-3.0-or-later. See the [[LICENSE]] and [[COPYING.LESSER]] files for details.

## Dictionary

A dictionary file must follow this structure:

```tsv
orthography  phonetic_transcription, phonetic_transcription
```

## Syntax

Paddler has a couple simple tokens that can be found in [segment.rs](./src/segment.rs#L26):

```rust
pub enum Segment {
    IPA(IPA),
    FeatureSet(FeatureSet),
    Any,
    Stress(bool), // bool marks absence
    SecondaryStress(bool),
    Syllable(bool),
}
```

## Segments

- IPA:
  - An IPA entry, that must exist in [[ipa_base.csv]]
    - An IPA token may be modified with Modifiers defined in [[./src/modifier.rs]], these will update the featureset of the given IPA.
- FeatureSet
  - A list of [Features](./src/ipa.rs#L11), put inside square brackets with signs (i.e. [consonant -voice, +sonorant])
- Any
  - The '_' character that matches any other segment
- Stress
  - IPA representation(ˈ) or for simpler use, a single tick (')
  - `~` character before this segment marks the segment as absent.
- SecondaryStress
  - The 'ˌ' character
  - `~` character before this segment marks the segment as absent.
- Syllable
  - The dot character(.) that represents a syllable boundary
  - `~` character before this segment marks the segment as absent.

## Usage

```bash
Usage: paddler [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  

Options:
  -d, --dict <DICT>  [default: en_US.txt]
  -h, --help         Print help
  -V, --version      Print version
  --segmentize       Prints the segments
  --strict           Runs the pattern matching in strict mode
```

## Strict Mode

Strict mode refers to the fact that skippable segments should not be skipped.

By default, Paddler operates on non-strict mode, which allows the SyllableBoundary, Stress, and SecondaryStress segments in words to be skipped. This means both: "#a.b", and "#ab"  will match /a.b/; but "#a.b" won't match /ab/, as the Segments in the pattern cannot be skipped.

In strict mode however, "#ab" will only match /ab/. Strict mode removes the skipping feature.

## Absence Marker

Certain segments (SyllableBoundary, Stress, SecondaryStress) can be marked as absent via putting the `~` symbol before the segment. This makes it so that Paddler checks for the absence of that segment. For example: "#z" will match words starting with /z/ which may also have a stress marker before it, while "#~'z" will match words that are starting with /z/ and don't have a stress marker before it. 

## Examples

```bash
$ paddler "#[cons][-cons]ŋk#"
banc, ˈbæŋk
bank, ˈbæŋk
banke, ˈbæŋk
banque, ˈbæŋk
behnke, ˈbɛŋk
benke, ˈbɛŋk
```

Or with a custom dictionary:

```bash
$ paddler "#[cons][-cons]ŋk# --dict=cmudict-ipa.tsv"
BANC, 'bæŋk
BANK, 'bæŋk
BANKE, 'bæŋk
BANQUE, 'bæŋk
BEHNKE, 'bɛŋk
BENKE, 'bɛŋk
```

## Acknowledgements

- [PanPhon](https://github.com/dmort27/panphon/), for [[ipa_base.csv]] and [diacritics / modifiers](./src/modifier.rs).
- [ipa-dict](https://github.com/open-dict-data/ipa-dict/), for providing ipa transcriptions of words in various languages, which have been utilized for testing, and `en_US.txt` is included by default.
