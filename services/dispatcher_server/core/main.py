from concurrent import futures
from pathlib import Path

import click
import click_config_file
import grpc
import toml
from dispatcher_server.core.kyc_params import KYCParams
from dispatcher_server.core.service import DispatcherService
from dispatcher_server.proto_generated.dispatcher_pb2_grpc import (
    add_DispatcherServicer_to_server,
)


def toml_config_provider(file_path, cmd_name):
    with open(file_path) as cfg_data:
        return toml.load(cfg_data)[cmd_name]


@click.command(name="dispatcher_server")
@click.option(
    "--verifier-host",
    type=str,
    help="Hostname to locate verifier",
    default="http://127.0.0.1",
)
@click.option(
    "--verifier-port",
    type=str,
    help="Port on host to access verifier",
    default="50053",
)
@click.option(
    "--port",
    "-p",
    type=str,
    help="Port to deploy this service to",
    default="50052",
)
@click.option(
    "--kyc-file",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    help="Path to KYC parameters configuration file",
    required=True,
)
@click.option(
    "--provers-file",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    help="Path to a file with available provers info",
    required=True,
)
@click_config_file.configuration_option(
    provider=toml_config_provider,
    implicit=False,
    cmd_name="dispatcher_server",
)
def main(
    verifier_host: str,
    verifier_port: str,
    port: str,
    kyc_file: Path,
    provers_file: Path,
):
    kyc_params = KYCParams(kyc_file)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    add_DispatcherServicer_to_server(
        DispatcherService(
            kyc_params,
            verifier_host,
            verifier_port,
            provers_file,
        ),
        server,
    )
    server.add_insecure_port(f"[::]:{port}")

    server.start()
    server.wait_for_termination()
