import clickhouse_driver


class ClickhouseConnector:
    def __init__(
        self,
        db_host: str,
        db_port: str,
        db_name: str,
        db_user: str,
        db_pwd: str,
    ) -> None:
        self.db_host = db_host
        self.db_port = db_port
        self.db_name = db_name
        self.db_user = db_user
        self.db_pwd = db_pwd

        self.client = clickhouse_driver.Client(
            host=self.db_host,
            port=self.db_port,
            database=self.db_name,
            user=self.db_user,
            password=self.db_pwd,
        )

    def query(self, query: str) -> list:
        data = self.client.execute(query)

        if not data:
            return []
        return data[0]
