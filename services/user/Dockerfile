FROM python:3.10.8

COPY user /user

RUN pip install -r /user/requirements.txt

ENTRYPOINT [ "python", "-m", "user" ]
