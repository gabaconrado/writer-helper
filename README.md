# Writing helper

Simple Rust CLI example, using the crate [clap](https://crates.io/crates/clap).

This software will help you check your writings structure by providing you the
following information:

- Line count
- Word count
- Sections count

It will also provide the following actions:

- Word search: Returns the word count and position(line number, word number 
on line) for a given word.

## Usage

To check general information about a file, just invoke the binary passing it as
the first argument.

```bash
whelper input_file
```

To check information about a specific word in the file, pass it through the 
`--word` or `-w` option:

```bash
whelper input_file --word word
# Or with the short flag
whelper input_file -w word
```

## Additional information

To check the help text

```bash
whelper --help
# Or with the short flag
whelper -h
```

To run it in verbose mode

```bash
whelper --verbose
# Or with the short flag
whelper -v
```

## Information meaning

- Sections: All lines that starts with `#`;
- Lines: Number of `/n` in the text;
- Words: Blocks of characters delimited by ` `, `,` or `.`. Does not include
words in section lines.
