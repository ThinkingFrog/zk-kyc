import pathlib
from concurrent import futures

import click
import click_config_file
import grpc
import toml
from verifier_server.core.service import VerifierService
from verifier_server.proto_generated.verifier_pb2_grpc import (
    add_VerifierServicer_to_server,
)


def toml_config_provider(file_path, cmd_name):
    with open(file_path) as cfg_data:
        return toml.load(cfg_data)[cmd_name]


@click.command(name="verifier_server")
@click.option(
    "--port",
    "-p",
    type=str,
    help="Port to deploy this service on",
    default="50053",
)
@click.option(
    "--verifier-lib",
    type=click.Path(
        exists=True, dir_okay=False, executable=True, path_type=pathlib.Path
    ),
    help="Path to C shared library used to call verifier",
    required=True,
)
@click_config_file.configuration_option(
    provider=toml_config_provider,
    implicit=False,
    cmd_name="verifier_server",
)
def main(
    port: str,
    verifier_lib: pathlib.Path,
):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    add_VerifierServicer_to_server(VerifierService(verifier_lib), server)
    server.add_insecure_port(f"[::]:{port}")

    server.start()
    server.wait_for_termination()
