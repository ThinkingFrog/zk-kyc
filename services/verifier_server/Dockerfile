FROM python:3.10.8

COPY verifier_server /verifier_server

RUN pip install -r /verifier_server/requirements.txt

ENTRYPOINT [ "python", "-m", "verifier_server" ]
