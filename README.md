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
    Stress,
    SecondaryStress,
    Syllable,
}
```

### Tokens

- IPA:
  - An IPA entry, that must exist in [[ipa_base.csv]]
    - An IPA token may be modified with Modifiers defined in [[./src/modifier.rs]], these will update the featureset of the given IPA.
- FeatureSet
  - A list of [Features](./src/ipa.rs#L11), put inside square brackets with signs (i.e. [consonant -voice, +sonorant])
- Any
  - The '_' character that matches any other segment
- Stress
  - IPA representation(ˈ) or for simpler use, a single tick (')
- SecondaryStress
  - The 'ˌ' character
- Syllable
  - The dot character(.) that represents a syllable boundary

## Usage

```bash
Usage: paddler [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  

Options:
  -d, --dict <DICT>  [default: en_US.txt]
  -h, --help         Print help
  -V, --version      Print version
```

### Examples

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
