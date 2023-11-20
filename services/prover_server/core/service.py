import ctypes

from db_connector.clickhouse_connector import ClickhouseConnector
from prover_server.core.config import Config
from prover_server.proto_generated.prover_pb2 import ProofRequest, ProofResponse
from prover_server.proto_generated.prover_pb2_grpc import ProverServicer


class ProverService(ProverServicer):
    def __init__(self, cfg: Config) -> None:
        super().__init__()

        self.cfg = cfg

        self.db_conn = ClickhouseConnector(
            self.cfg.db_host,
            self.cfg.db_port,
            self.cfg.db_name,
            self.cfg.db_user,
            self.cfg.db_pwd,
        )

    def get_proof(self, request: ProofRequest, context):
        # Retrieve data from DB by user_id
        data = self.db_conn.query(
            f"SELECT USER_ID, AGE, COUNTRY_ID FROM test_dataset WHERE USER_ID = {request.user_id}"
        )
        if not data:
            return ProofResponse(
                proof=bytes(),
                status_code=1,
                status_msg=f"Couldn't process query on VDR database with user ID {request.user_id}",
            )
        data = data[1:3]

        # Call C bindings of rust lib to create proof
        prover_lib = ctypes.CDLL(self.cfg.prover_lib)

        # Make simple type checks for C function
        prover_lib.create_proof.argtypes = [
            ctypes.c_ulong,
            ctypes.c_ulong,
            ctypes.c_ulong,
            ctypes.POINTER(ctypes.c_ulong),
            ctypes.c_ulong,
            ctypes.POINTER(ctypes.c_ulong),
        ]
        prover_lib.create_proof.restype = ctypes.POINTER(ctypes.c_uint8)

        # Assign variables with appropriate C types
        user_age = ctypes.c_ulong(data[0])
        user_country = ctypes.c_ulong(data[1])
        public_age = ctypes.c_ulong(request.age_input)

        public_countries = request.country_inputs
        public_counties_ctypes = (ctypes.c_ulong * len(public_countries))(
            *public_countries
        )

        proof_len = ctypes.c_ulong()

        # Call C function to create proof
        proof = prover_lib.create_proof(
            user_age,
            user_country,
            public_age,
            public_counties_ctypes,
            len(public_countries),
            ctypes.pointer(proof_len),
        )

        if proof_len.value == 0:
            return ProofResponse(
                proof=bytes(),
                status_code=1,
                status_msg=f"Error composing proof for user ID {request.user_id}, possible ZK backend library malfunction",
            )

        proof_bytes = bytes(proof[idx] for idx in range(proof_len.value))

        return ProofResponse(
            proof=proof_bytes,
            status_code=0,
            status_msg="SUCCESS",
        )
