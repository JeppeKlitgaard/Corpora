from pydantic import BaseModel, Field, HttpUrl, FileUrl
from semver import VersionInfo
from typing import Any


class SemanticVersion(VersionInfo):
    @classmethod
    def __get_validators__(cls):
        yield cls.parse

    @classmethod
    def __modify_schema__(cls, field_schema):
        field_schema.update(examples=["1.0.2", "2.15.3-alpha", "21.3.15-beta+12345"])


class CorpusSource(BaseModel):
    name: str
    url: HttpUrl | FileUrl
    version: SemanticVersion
    license: str = Field(max_length=256, description="License, ideally SPDX format")


class Corpus(BaseModel):
    name: str = Field(max_length=256)
    id: str = Field(max_length=256)
    language: str
    version: SemanticVersion
    extra_metadata: dict[str, Any]
    source: CorpusSource
    ngrams: dict[str, dict[str, float]]
