import click
import click_config_file
import grpc
import toml
from flask import Flask, jsonify, render_template, request
from user.proto_generated.dispatcher_pb2 import UserRequest
from user.proto_generated.dispatcher_pb2_grpc import DispatcherStub

app = Flask(__name__)
dispatcher_host = ""
dispatcher_port = ""


def toml_config_provider(file_path, cmd_name):
    with open(file_path) as cfg_data:
        return toml.load(cfg_data)[cmd_name]


@app.route("/")
def index():
    return render_template("index.html")


@app.route("/send_request", methods=["POST"])
def send_request():
    print(request)
    vdr_id = int(request.form["vdr_id"])
    user_id = int(request.form["user_id"])

    with grpc.insecure_channel(f"{dispatcher_host}:{dispatcher_port}") as channel:
        stub = DispatcherStub(channel)
        kyc_response = stub.request_verification(
            UserRequest(vdr_id=vdr_id, user_id=user_id)
        )

    if kyc_response.status_code != 0:
        return jsonify({"response": kyc_response.status_msg})

    response_msg = "Passed" if kyc_response.kyc_result else "Didn't pass"
    return jsonify({"response": response_msg})


@click.command(name="user")
@click.option(
    "--host",
    type=str,
    help="Hostname to locate deployed Dispatcher Server",
    default="localhost",
)
@click.option(
    "--port",
    type=str,
    help="Port to access deployed Dispatcher Server",
    default="50052",
)
@click_config_file.configuration_option(
    provider=toml_config_provider,
    implicit=False,
    cmd_name="user",
)
def main(
    host: str,
    port: str,
):
    global dispatcher_host
    dispatcher_host = host

    global dispatcher_port
    dispatcher_port = port

    app.run(debug=True)
