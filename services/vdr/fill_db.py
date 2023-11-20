import csv
import logging
import warnings

from clickhouse_driver import Client

client_CH = Client(host="localhost", user="default", port="19000", password="")


def logger():
    logs = logging.getLogger()
    logs.setLevel(logging.INFO)
    logging.basicConfig(level=logging.INFO)
    warnings.filterwarnings(action="ignore")

    return logs


logs = logger()


def execute_query(sql: str, params=None):
    output = f"\nout :\n{sql}\n"
    logs.info(output)

    client_CH.execute(sql, params, types_check=True)


def get_create_test_table():
    name = "test_dataset"
    cols = """
                (
                USER_ID UInt32,
                AGE UInt32,
                COUNTRY_ID UInt32,
                )
                """

    return f"""
            CREATE TABLE IF NOT EXISTS {name}
                {cols}
            ENGINE = MergeTree()
            PARTITION BY
                tuple()
            ORDER BY
                USER_ID
            SETTINGS index_granularity = 8192;
            """


def main():
    schema = {
        "USER_ID": int,
        "AGE": int,
        "COUNTRY_ID": int,
    }

    bypass = lambda x: x

    create_table = get_create_test_table()
    execute_query(create_table)

    with open("db_test_data.csv") as f:
        gen = (
            {k: schema.get(k, bypass)(v) for k, v in row.items()}
            for row in csv.DictReader(f)
        )
        client_CH.execute("INSERT INTO test_dataset VALUES", gen)

    logs.info(client_CH.execute("SELECT * FROM test_dataset LIMIT 50"))


if __name__ == "__main__":
    main()
