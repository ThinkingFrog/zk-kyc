import pathlib
from concurrent import futures

import click
import click_config_file
import grpc
import toml
from prover_server.core.config import Config
from prover_server.core.service import ProverService
from prover_server.proto_generated.prover_pb2_grpc import add_ProverServicer_to_server


def toml_config_provider(file_path, cmd_name):
    with open(file_path) as cfg_data:
        return toml.load(cfg_data)[cmd_name]


@click.command(name="prover_server")
@click.option(
    "--port",
    "-p",
    type=str,
    help="Port to deploy this service on",
    default="50051",
)
@click.option(
    "--db-host",
    type=str,
    help="Hostname to locate deployed VDR DB",
    default="localhost",
)
@click.option(
    "--db-port",
    type=str,
    help="Port to access deployed VDR DB",
    default="19000",
)
@click.option(
    "--db-name",
    type=str,
    help="Deployed VDR DB name",
    required=True,
)
@click.option(
    "--db-user",
    type=str,
    help="User to login to deployed VDR DB with",
    required=True,
)
@click.option(
    "--db-pwd",
    type=str,
    help="Password for given VDR DB user",
    required=True,
)
@click.option(
    "--prover-lib",
    type=click.Path(
        exists=True, dir_okay=False, executable=True, path_type=pathlib.Path
    ),
    help="Path to C shared library used to call prover",
    required=True,
)
@click_config_file.configuration_option(
    provider=toml_config_provider,
    implicit=False,
    cmd_name="prover_server",
)
def main(
    port: str,
    db_host: str,
    db_port: str,
    db_name: str,
    db_user: str,
    db_pwd: str,
    prover_lib: pathlib.Path,
):
    cfg = Config(db_host, db_port, db_name, db_user, db_pwd, prover_lib)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    add_ProverServicer_to_server(ProverService(cfg), server)
    server.add_insecure_port(f"[::]:{port}")

    server.start()
    server.wait_for_termination()
