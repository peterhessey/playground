# Spanishdict List CSV Extractor

Tool for pulling public vocab lists from [spanishdict.com](https://spanishdict.com/) into a CSV file. Useful for importing them into Anki!

## How to use

```shell
uv run main.py LIST_ID
```

Where `LIST_ID` is the ID in the spanishdict URL, e.g. `8147865/lista-de-peru` for my Peru word list.

The list will need to be public as this tool does not handle authentication against spanishdict for private lists.
