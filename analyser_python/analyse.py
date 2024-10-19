from collections import deque, Counter, OrderedDict
from operator import itemgetter
import itertools
from typing import Any, Self
from pydantic import BaseModel, Field, field_serializer, computed_field

def _slice_deque[T](d: deque[T], start: int, stop: int, step: int = 1) -> list[T]:
    """
    Slice operation for deques.

    Taken from https://stackoverflow.com/a/54974253
    """
    d.rotate(-start)
    slice = list(itertools.islice(d, 0, stop-start, step))
    d.rotate(start)
    return slice

def _sort_dict(d: dict[Any, Any]) -> OrderedDict[Any, Any]:
    return OrderedDict(sorted(d.items(), key=itemgetter(1), reverse=True))

CHAR_SETS = {
    "LATIN_ALPHAS": "abcdefghijklmnopqrstuvwxyz",
    "NORDIC_ALPHAS": "æøå",
    "WHITESPACE": " \t\n",
    "NUMERIC": "0123456789",
    "PUNCTUATION": ",.:;?!¡¿‽",
    "BRACKETS": "()[]{}<>",
    "MATH": "+-=%*",
    "QUOTATION": "\"'`",
    "SYMBOLS": "~@#_^$&|",
}

CHARS_TO_RECORD = "".join(CHAR_SETS.values())


class NGram(BaseModel):
    """
    Collection of N-grams for a particular N.
    """
    counts: Counter[str] = Field(default_factory=lambda: Counter())

    def __add__(self, other: Self) -> Self:
        return NGram(counts=self.counts + other.counts)


    def apply_filter(self, allowed_chars: str) -> tuple[Self, Counter[str]]:
        """
        Returns a tuple containing the filtered NGram and a counter of
        filtered characters.
        """
        filtered_chars = Counter()
        filtered_counts = self.counts.copy()

        for key in self.counts.keys():
            for char in key:
                if char not in allowed_chars:
                    del filtered_counts[key]
                    filtered_chars[char] += 1

        filtered_ngram = NGram(counts=filtered_counts)

        return filtered_ngram, filtered_chars

    @field_serializer("counts")
    def serialize_counts(self, counts: Counter[str], _info):
        return _sort_dict(counts)


class NGrams(BaseModel):
    N: int
    ngram_counts: dict[int, NGram]
    filtered_chars: Counter[str]

    def __add__(self, other: Self) -> Self:
        assert self.N == other.N

        summed_counts = {
            n: ngram + other.ngram_counts[n] for n, ngram in self.ngram_counts.items()
        }

        return NGrams(
            N=self.N,
            ngram_counts=summed_counts,
            filtered_chars=self.filtered_chars + other.filtered_chars
        )

    @classmethod
    def new_from_N(cls, N: int) -> Self:
        ngram_counts = {
            n: NGram()
            for n in range(1, N+1)
        }

        return cls(
            N=N,
            ngram_counts = ngram_counts,
            filtered_chars = Counter()
        )

    @computed_field
    def ngram_frequencies(self) -> dict[int, dict[str, float]]:
        frequencies = {
            n: _sort_dict({
                k: count / ngram.counts.total() for k, count in ngram.counts.items()
            }) for n, ngram in self.ngram_counts.items()
        }

        return frequencies

    def apply_filter(self, allowed_chars: str) -> Self:
        """
        Returns an NGram object that has been filtered to only contain allowed_chars.
        """
        filtered_ngram_counts = {}
        filtered_chars = self.filtered_chars.copy()

        for n, ngram in self.ngram_counts.items():
            _ngrams, _filtered_chars = ngram.apply_filter(allowed_chars=allowed_chars)

            filtered_ngram_counts[n] = _ngrams
            filtered_chars += _filtered_chars

        return NGrams(N = self.N, ngram_counts=filtered_ngram_counts, filtered_chars=filtered_chars)


class Analyser:
    def __init__(self, lower_chars: bool = True, ngram_n: int = 3) -> None:
        self.lower_chars = lower_chars
        self.ngrams = NGrams.new_from_N(N=ngram_n)

    def ingest(self, text: str) -> None:
        self.ngram_deque: deque[str] = deque([], maxlen=self.ngrams.N)

        if self.lower_chars:
            text = text.lower()

        for char in text:
            self.ngram_deque.append(char)

            for n, ngram in self.ngrams.ngram_counts.items():
                N = self.ngrams.N
                ngram_l: list[str] = _slice_deque(self.ngram_deque, N - n, N)

                if len(ngram_l) != n:
                    continue

                entry = "".join(ngram_l)
                ngram.counts[entry] += 1


class WortschatzAnalyser(Analyser)
    # def to_counts():
    #     ...

    # def to_ngrams(self) -> dict[str, dict[str, float]]:
    #     ngrams: dict[str, dict[str, float]] = {}
    #     for n, counter in self.ngram_counters.items():
    #         n_ngrams_dict = { k: count / counter.total() for k, count in counter.items()}

    #         sorted_dict = OrderedDict(sorted(n_ngrams_dict.items(), key=itemgetter(1), reverse=True))
    #         ngrams[str(n)] = sorted_dict

    #     return ngrams
