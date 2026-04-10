# Language Detector

A fast command-line tool written in Rust for detecting the language of subtitle files.

It reads subtitle text, strips timing and formatting markup, tokenizes the spoken dialogue, compares it against built-in language word lists, and reports the most likely match.

The project supports **80 languages** and can output results as a human-readable table, JSON, or CSV.

## Features

- Detects the most likely language from subtitle files
- Supports **80 built-in languages**
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
  - SSA/ASS override tags like `{"..."}`
  - HTML-like tags such as `<i>` and `<font>`
- Multithreaded scanning across all supported languages
- Multiple output formats:
  - `table`
  - `json`
  - `csv`

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
4. **Compare tokens against built-in language lexicons**
5. **Score every language** by number of matches
6. **Return ranked results** with the top match shown as the detected language

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
╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗
║                                 Language Detection Results                                           ║
╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣
║  File: sub_spa.ass                                                                                   ║
║  Total words parsed: 2551                                                                            ║
╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                      ║
║  ✓ DETECTED LANGUAGE: Spanish (Latin America)                                                        ║
║    ISO 639-1: es                                                                                     ║
║    ISO 639-2: spa                                                                                    ║
║    BCP 47:    es-419                                                                                 ║
║    Confidence: 40.73%  (1039 / 2551 words matched)                                                   ║
║                                                                                                      ║
╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣
║  All language scores (top 10):                                                                       ║
╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣
║  #   Language                     639-1   639-2   BCP 47         Matches      Score                  ║
║  --- ---------------------------- ------- ------- -------------- ------------ ------------           ║
║  1   Spanish (Latin America)      es      spa     es-419         1039         40.73%                 ║
║  2   Spanish (Spain)              es      spa     es-ES          1035         40.57%                 ║
║  3   Catalan                      ca      cat     -              537          21.05%                 ║
║  4   Galician                     gl      glg     -              533          20.89%                 ║
║  5   Portuguese (Portugal)        pt      por     pt-PT          474          18.58%                 ║
║  6   Portuguese (Brazil)          pt      por     pt-BR          465          18.23%                 ║
║  7   French                       fr      fra     -              436          17.09%                 ║
║  8   Italian                      it      ita     -              400          15.68%                 ║
║  9   Romanian                     ro      ron     -              289          11.33%                 ║
║  10  Hungarian                    hu      hun     -              202          7.92%                  ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝
```

### JSON

```json
{
	"file": "sub_spa.ass",
	"total_words": 2551,
	"detected": {
		"language": "Spanish (Latin America)",
		"iso_639_1": "es",
		"iso_639_2": "spa",
		"bcp47": "es-419",
		"matched_words": 1039,
		"confidence": 0.4073
	},
	"scores": [
		{
			"rank": 1,
			"language": "Spanish (Latin America)",
			"iso_639_1": "es",
			"iso_639_2": "spa",
			"bcp47": "es-419",
			"matched_words": 1039,
			"total_words": 2551,
			"confidence": 0.4073
		},
		{
			"rank": 2,
			"language": "Spanish (Spain)",
			"iso_639_1": "es",
			"iso_639_2": "spa",
			"bcp47": "es-ES",
			"matched_words": 1035,
			"total_words": 2551,
			"confidence": 0.4057
		},
		{
			"rank": 3,
			"language": "Catalan",
			"iso_639_1": "ca",
			"iso_639_2": "cat",
			"bcp47": null,
			"matched_words": 537,
			"total_words": 2551,
			"confidence": 0.2105
		}
	]
}
```

### CSV

```csv
rank,language,iso_639_1,iso_639_2,bcp47,matched_words,total_words,confidence
1,Spanish (Latin America),es,spa,es-419,1039,2551,0.4073
2,Spanish (Spain),es,spa,es-ES,1035,2551,0.4057
3,Catalan,ca,cat,-,537,2551,0.2105
```

## Detection strategy

This project uses lightweight lexicon-based language detection rather than a large statistical or neural model.

That gives it a few advantages:

- fast
- no external dependencies required at runtime
- easy to inspect and extend
- predictable output

It also means performance depends heavily on:

- subtitle quality
- how much spoken text is available
- overlap between related languages
- the quality and coverage of each language word list

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
   - a representative common-word list
3. Register the language in the `languages/mod.rs`

The better the common-word list, the better the detector tends to perform.

## Exit behavior

The program exits with an error when:

- no file path is provided
- an unknown option is used
- an unknown output format is used
- the file cannot be read
- no usable words are found after cleaning
