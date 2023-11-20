import csv
import itertools
from pathlib import Path


def generator():
    with Path("db_test_data.csv").open("w") as csvf:
        writer = csv.writer(csvf)
        writer.writerow(["USER_ID", "AGE", "COUNTRY_ID"])

        for user_id, (age, country_id) in enumerate(
            itertools.product(range(151), range(1, 81))
        ):
            writer.writerow([user_id + 1, age, country_id])


if __name__ == "__main__":
    generator()
