FROM python:3.10.8

COPY dispatcher_server /dispatcher_server

RUN pip install -r /dispatcher_server/requirements.txt

ENTRYPOINT [ "python", "-m", "dispatcher_server" ]
