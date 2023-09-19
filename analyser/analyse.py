from pathlib import Path
from analyser import analyse_wortschatz

root_path = Path(__file__).parent.resolve()
sources_path = root_path / "sources"
wortschatz_path = sources_path / "wortschatz"

english_archives = list(wortschatz_path.glob("eng_*.tar.gz"))[:5]

print(f"Analysing: {english_archives}")

ngrams = analyse_wortschatz(
    english_archives,
    [
        1,
        2,
        3,
        4,
    ],
)
