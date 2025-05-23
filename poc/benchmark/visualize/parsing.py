import json

from pydantic import BaseModel


class Case(BaseModel):
    resource_count: int
    kind_count: int
    duration_get_latest: list[int]
    duration_direct: list[int]


def parse_line(line: str) -> Case:
    raw = json.loads(line)
    return Case.model_validate(raw)


def parse_results_file(path: str) -> list[Case]:
    with open(path, "r") as infile:
        lines = infile.readlines()

    cases = []
    for line in lines:
        cases.append(parse_line(line))

    return cases
