import json
from pathlib import Path
from typing import List


class KYCParams:
    minimum_age: int
    allowed_countries: List[int]

    def __init__(self, params_file: Path) -> None:
        with params_file.open("r") as pf:
            pf_dict = json.load(pf)

        self.minimum_age = pf_dict["minimum_age"]
        self.allowed_countries = pf_dict["countries"]
