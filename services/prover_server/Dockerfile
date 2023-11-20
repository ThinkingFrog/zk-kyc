FROM python:3.10.8

COPY prover_server /prover_server
COPY db_connector /db_connector

RUN pip install -r /prover_server/requirements.txt
RUN pip install -r /db_connector/requirements.txt

ENTRYPOINT [ "python", "-m", "prover_server" ]
