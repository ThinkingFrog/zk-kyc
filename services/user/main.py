import click
import click_config_file
import grpc
import toml
from user.proto_generated.dispatcher_pb2 import UserRequest
from user.proto_generated.dispatcher_pb2_grpc import DispatcherStub


def toml_config_provider(file_path, cmd_name):
    with open(file_path) as cfg_data:
        return toml.load(cfg_data)[cmd_name]


@click.command(name="user")
@click.option(
    "--vdr-id",
    type=int,
    help="VDR ID in which user's personal data is stored",
    required=True,
)
@click.option(
    "--user-id",
    type=int,
    help="User ID in given VDR",
    required=True,
)
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
    vdr_id: int,
    user_id: int,
    host: str,
    port: str,
):
    with grpc.insecure_channel(f"{host}:{port}") as channel:
        stub = DispatcherStub(channel)
        kyc_response = stub.request_verification(
            UserRequest(vdr_id=vdr_id, user_id=user_id)
        )

    if kyc_response.status_code != 0:
        print(f"Error occurred: {kyc_response.status_msg}")
    else:
        print(f"KYC passed: {kyc_response.kyc_result}")
