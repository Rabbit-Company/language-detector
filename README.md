# Language Detector

A fast command-line tool written in Rust for detecting the language of subtitle files.

It reads subtitle text, strips timing and formatting markup, tokenizes the spoken dialogue, compares it against built-in language word lists, and reports the most likely match.

The project supports **80 languages** and can output results as a human-readable table, JSON, or CSV.

## Features

- Detects the most likely language from subtitle files
- Supports **80 built-in languages**, including regional variants
- Works with common subtitle and plain-text formats:
  - `.srt`
  - `.ass`
  - `.ssa`
  - `.sub`
  - `.txt`
- Cleans subtitle markup before detection
- Handles:
  - SRT sequence numbers and timestamps
  - SSA/ASS dialogue lines and metadata
  - SSA/ASS override tags like `{"…"}`
  - HTML-like tags such as `<i>` and `<font>`
- **Two‑pass detection**:
  - Pass 1 – language identification using shared common words
  - Pass 2 – variant disambiguation using weighted dialect‑specific markers
- Multithreaded scanning across all supported languages
- Multiple output formats:
  - `table`
  - `json`
  - `csv`
- **Debug mode** to inspect exactly which words contributed to a language’s score

## Installation

```bash
# Download the binary
wget https://github.com/Rabbit-Company/language-detector/releases/latest/download/language-detector-$(uname -m)-gnu
# Set file permissions
sudo chmod 777 language-detector-$(uname -m)-gnu
# Place the binary to `/usr/local/bin`
sudo mv language-detector-$(uname -m)-gnu /usr/local/bin/language-detector
# Start language detector
language-detector
```

## Upgrade

```bash
# Download Language Detector
wget https://github.com/Rabbit-Company/language-detector/releases/latest/download/language-detector-$(uname -m)-gnu
sudo chmod 777 language-detector-$(uname -m)-gnu
sudo mv language-detector-$(uname -m)-gnu /usr/local/bin/language-detector
```

## How it works

The detector follows a simple pipeline:

1. **Read the subtitle file**
2. **Strip non-dialogue content** such as timestamps, metadata, and markup
3. **Tokenize the remaining text**
   - whitespace-based tokenization for space-delimited languages
   - character and bigram tokenization for scripts that usually do not separate words with spaces
4. **Two‑pass scoring**
   - **Pass 1** compares tokens against shared common_words for every language. This identifies the broad language family (e.g., Spanish, Portuguese, Chinese).
   - **Pass 2** re‑ranks language variants (e.g., es-419 vs es-ES) using weighted_words—dialect‑specific spelling, grammar, and vocabulary that carry a higher score.
5. **Return ranked results** with the top match shown as the detected language

## Project structure

```text
main.rs      CLI entry point and orchestration
cleaner.rs   Subtitle cleanup and tokenization
scanner.rs   Language scanning and scoring
output.rs    Table, JSON, and CSV renderers
languages/   Built-in language catalogue and word lists
```

## Usage

```bash
language-detector [OPTIONS] <FILE>
```

### Arguments

- `<FILE>` — path to a file

### Options

- `-f, --format <FORMAT>` — output format: `table`, `json`, or `csv`
- `-d, --debug <LANG>` — debug mode: show detailed match info for a language (accepts name, ISO code, or BCP 47 tag)
- `-V, --version` — print version information
- `-h, --help` — show help

## Examples

Detect the language of a file:

```bash
language-detector movie.srt
```

Output JSON:

```bash
language-detector -f json movie.srt
```

Output CSV:

```bash
language-detector --format csv movie.srt
```

Save JSON to a file:

```bash
language-detector -f json movie.srt > result.json
```

## Example output

### Table

```text
┌────────────────────────────────────────────────┬────────────────────┐
│                      File                      │ Total words parsed │
├────────────────────────────────────────────────┼────────────────────┤
│ sub_spa.ass                                    │ 2124               │
└────────────────────────────────────────────────┴────────────────────┘

┌─────────────────────────┬───────────┬───────────┬────────┬─────────────────────┬────────────────┐
│    Detected language    │ ISO 639-1 │ ISO 639-2 │ BCP 47 │     Confidence      │ Weighted score │
├─────────────────────────┼───────────┼───────────┼────────┼─────────────────────┼────────────────┤
│ Spanish (Latin America) │ es        │ spa       │ es-419 │ 43.69% (928 / 2124) │ 31.00          │
└─────────────────────────┴───────────┴───────────┴────────┴─────────────────────┴────────────────┘

┌────┬─────────────────────────┬───────┬───────┬────────┬─────────┬────────────┬──────────┐
│ #  │        Language         │ 639-1 │ 639-2 │ BCP 47 │ Matches │ Confidence │ Weighted │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 1  │ Spanish (Latin America) │ es    │ spa   │ es-419 │ 928     │ 43.69%     │ 31.00    │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 2  │ Spanish (Spain)         │ es    │ spa   │ es-ES  │ 928     │ 43.69%     │ 5.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 3  │ Catalan                 │ ca    │ cat   │ -      │ 507     │ 23.87%     │ 0.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 4  │ Galician                │ gl    │ glg   │ -      │ 469     │ 22.08%     │ 0.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 5  │ Portuguese (Portugal)   │ pt    │ por   │ pt-PT  │ 414     │ 19.49%     │ 140.00   │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 6  │ Portuguese (Brazil)     │ pt    │ por   │ pt-BR  │ 414     │ 19.49%     │ 45.00    │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 7  │ French                  │ fr    │ fra   │ -      │ 383     │ 18.03%     │ 0.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 8  │ Italian                 │ it    │ ita   │ -      │ 323     │ 15.21%     │ 0.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 9  │ Romanian                │ ro    │ ron   │ -      │ 259     │ 12.19%     │ 0.00     │
├────┼─────────────────────────┼───────┼───────┼────────┼─────────┼────────────┼──────────┤
│ 10 │ Hungarian               │ hu    │ hun   │ -      │ 176     │ 8.29%      │ 0.00     │
└────┴─────────────────────────┴───────┴───────┴────────┴─────────┴────────────┴──────────┘
```

### JSON

```json
{
	"file": "sub_spa.ass",
	"total_words": 2124,
	"detected": {
		"language": "Spanish (Latin America)",
		"iso_639_1": "es",
		"iso_639_2": "spa",
		"bcp47": "es-419",
		"matched_words": 928,
		"confidence": 0.4369,
		"weighted_score": 31.0
	},
	"scores": [
		{
			"rank": 1,
			"language": "Spanish (Latin America)",
			"iso_639_1": "es",
			"iso_639_2": "spa",
			"bcp47": "es-419",
			"matched_words": 928,
			"total_words": 2124,
			"confidence": 0.4369,
			"weighted_score": 31.0
		},
		{
			"rank": 2,
			"language": "Spanish (Spain)",
			"iso_639_1": "es",
			"iso_639_2": "spa",
			"bcp47": "es-ES",
			"matched_words": 928,
			"total_words": 2124,
			"confidence": 0.4369,
			"weighted_score": 5.0
		},
		{
			"rank": 3,
			"language": "Catalan",
			"iso_639_1": "ca",
			"iso_639_2": "cat",
			"bcp47": null,
			"matched_words": 507,
			"total_words": 2124,
			"confidence": 0.2387,
			"weighted_score": 0.0
		}
	]
}
```

### CSV

```csv
rank,language,iso_639_1,iso_639_2,bcp47,matched_words,total_words,confidence,weighted_score
1,Spanish (Latin America),es,spa,es-419,928,2124,0.4369,31.0000
2,Spanish (Spain),es,spa,es-ES,928,2124,0.4369,5.0000
3,Catalan,ca,cat,-,507,2124,0.2387,0.0000
```

## Detection strategy

This project uses lightweight lexicon-based language detection rather than a large statistical or neural model.

That gives it a few advantages:

- fast
- no external dependencies required at runtime
- easy to inspect and extend
- predictable output

## Two‑pass scoring

The detector performs two passes over the tokenized text:

1. **Language identification** – every language is scored using its `common_words` (function words and high‑frequency neutral vocabulary). This groups related languages together.
2. **Variant disambiguation** – for languages that share the same ISO 639‑2 code (e.g., `spa`, `por`, `zho`), a second pass uses `weighted_words`. These are dialect‑specific spelling patterns, conjugations, and vocabulary that are strong signals for one variant over another.

The `Weighted score` column in the output shows the total from Pass 2. When two variants have identical Pass 1 match counts, the one with the higher weighted score is ranked higher.

## Debug mode

Use `--debug <LANG>` to see exactly which tokens matched `common_words` and `weighted_words`, along with their hit counts and contributions. This is invaluable for tuning word lists and understanding why a language scored the way it did.

## Limitations

This is a practical detector, not a full linguistic analyzer.

You may see weaker results when:

- the subtitle file is very short
- the text is mostly names, numbers, or sound effects
- two languages are very closely related
- subtitles are heavily mixed between multiple languages
- the lexicon for a language is too small or not representative

## Extending the project

To add or improve a language:

1. Add a language module in `languages/`
2. Provide:
   - English name
   - ISO 639-1 code
   - ISO 639-2 code
   - a `common_words` list (shared, neutral vocabulary)
   - a `weighted_words` list (dialect‑specific markers, if a variant)
3. Register the language in `languages/mod.rs`

The better the word lists, the better the detector performs.

## Exit behavior

The program exits with an error when:

- no file path is provided
- an unknown option is used
- an unknown output format is used
- the file cannot be read
- no usable words are found after cleaning
