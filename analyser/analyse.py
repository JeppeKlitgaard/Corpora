from pathlib import Path
from analyser import analyse_wortschatz

root_path = Path(__file__).parent.resolve()
sources_path = root_path / "sources"
wortschatz_path = sources_path / "wortschatz"

english_archives = list(wortschatz_path.glob("eng_*.tar.gz"))[:3]
# english_archives = [
#     Path("/home/jeppe/Code/keyboard/Corpora/analyser/sources/wortschatz/eng_wikipedia_2007_10K.tar.gz")
# ]

print(f"Analysing: {english_archives}")

ngrams = analyse_wortschatz(
    english_archives,
    [
        1,
        2,
        3,
    ],
)
