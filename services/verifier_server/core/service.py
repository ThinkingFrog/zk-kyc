import ctypes
from pathlib import Path

from verifier_server.proto_generated.verifier_pb2 import (
    VerificationRequest,
    VerificationResponse,
)
from verifier_server.proto_generated.verifier_pb2_grpc import VerifierServicer


class VerifierService(VerifierServicer):
    def __init__(self, verifier_lib: Path) -> None:
        super().__init__()

        self.verifier_lib_path = verifier_lib

    def verify_proof(self, request: VerificationRequest, context):
        # Call C bindings of rust lib to verify proof
        verifier_lib = ctypes.CDLL(self.verifier_lib_path)

        # Make simple type checks for C function
        verifier_lib.verify_proof.argtypes = [
            ctypes.c_ulong,
            ctypes.POINTER(ctypes.c_ulong),
            ctypes.c_ulong,
            ctypes.POINTER(ctypes.c_ubyte),
            ctypes.c_ulong,
        ]
        verifier_lib.verify_proof.restype = ctypes.c_ubyte

        # Assign variables with appropriate C types
        public_age = ctypes.c_ulong(request.public_age)

        public_countries = request.public_countries
        public_counties_ctypes = (ctypes.c_ulong * len(public_countries))(
            *public_countries
        )

        proof = request.proof
        proof_len = len(proof)

        proof_ctypes = (ctypes.c_ubyte * proof_len)(*proof)

        # Call C function to verify proof
        kyc_passed = verifier_lib.verify_proof(
            public_age,
            public_counties_ctypes,
            len(public_countries),
            proof_ctypes,
            proof_len,
        )

        kyc_passed = bool(kyc_passed)

        return VerificationResponse(kyc_passed=kyc_passed)
