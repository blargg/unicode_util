# unicode\_util

`unicode\_util` (alias `u`, in this document) is a simple, general purpose tool
for working with unicode characters. The focus is developer use cases, rather
than general use.

## Goals
* Fast
* Easy to use
* Developer oriented
    * supports command line usage

## How to use
It is recommended that you make an alias for this command.

```bash
alias u='unicode_util'
```

## Command line strings
In this example, we want to commit code using some unicode symbols. We can do
this by looking up the symbols with search, saving them, then writing them out
to the console as needed.

1. Lookup and Save
```
u search 'party'
```

This will open a command line app with the search results. From this list we can
navigate the list using vim-like key bindings, and select with the enter key.
This will open a prompt for the variable name to save. Enter `party` to save the
character.

2. Writing
With the characters we need saved, we can use them to write out commit message.

```bash
git commit -a -m "$(u get party) first commit!"
```

The `u get` command retrieves the character saved to "party". This will produce
the commit message `🎉 first commit`



## Related tools
* [Chars](https://github.com/antifuchs/chars): can look up information about
  individual unicode characters and code points.
