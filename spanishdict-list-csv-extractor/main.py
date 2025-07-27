import requests
from bs4 import BeautifulSoup
import csv
import click

SPANISH_DICT_LISTS_URL = "https://www.spanishdict.com/lists/"
TRANSLATION_PAIR_DIV_ID = "M51vAoht"


@click.command()
@click.argument(
    "list_id",
)
@click.option(
    "--output", "-o", default="spanish_translations.csv", help="Output CSV filename"
)
def main(
    list_id: str,
    output: str,
) -> None:
    url = SPANISH_DICT_LISTS_URL + list_id

    response = requests.get(url)
    response.raise_for_status()

    full_response_html = BeautifulSoup(response.text, "html.parser")

    translation_pair_divs = full_response_html.find_all(
        "div", class_=TRANSLATION_PAIR_DIV_ID
    )

    with open(output, "w", newline="", encoding="utf-8") as csvfile:
        writer = csv.writer(csvfile)
        for div in translation_pair_divs:
            spanish_div = div.find("div", class_="UO6pWUJR")
            english_div = div.find("div", class_="xLusdC9B")

            if spanish_div and english_div:
                spanish_text = spanish_div.get_text(strip=True)
                english_text = english_div.get_text(strip=True)
                writer.writerow([spanish_text, english_text])
            else:
                print(
                    f"WARNING: {spanish_div=}, {english_div=}. Expected contents for both. Skipping."
                )


if __name__ == "__main__":
    main()

