from dataclasses import dataclass
from pathlib import Path


@dataclass
class Config:
    db_host: str
    db_port: str
    db_name: str
    db_user: str
    db_pwd: str
    prover_lib: Path
