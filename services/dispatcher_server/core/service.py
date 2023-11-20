import json
from pathlib import Path
from typing import Dict

import grpc
from dispatcher_server.core.kyc_params import KYCParams
from dispatcher_server.proto_generated.dispatcher_pb2 import UserRequest, UserResponse
from dispatcher_server.proto_generated.dispatcher_pb2_grpc import DispatcherServicer
from dispatcher_server.proto_generated.prover_pb2 import ProofRequest
from dispatcher_server.proto_generated.prover_pb2_grpc import ProverStub
from dispatcher_server.proto_generated.verifier_pb2 import VerificationRequest
from dispatcher_server.proto_generated.verifier_pb2_grpc import VerifierStub


class DispatcherService(DispatcherServicer):
    kyc_params: KYCParams
    provers: Dict[int, str]
    node_host: str
    node_port: str

    def __init__(
        self,
        kyc_params: KYCParams,
        verifier_host: str,
        verifier_port: str,
        provers_file: Path,
    ) -> None:
        super().__init__()

        self.verifier_host = verifier_host
        self.verifier_port = verifier_port

        self.kyc_params = kyc_params

        with provers_file.open("r") as pf:
            pf_dict = json.load(pf)

        self.provers = dict(map(lambda x: (int(x[0]), x[1]), pf_dict.items()))

    def request_verification(self, request: UserRequest, context):
        # Step 1: Retrieve IDs from request and verify they are valid
        vdr_id = request.vdr_id
        user_id = request.user_id

        # Step 2: Create prover client and request proof by id
        if vdr_id not in self.provers.keys():
            return UserResponse(
                kyc_result=False,
                status_code=1,
                status_msg=f"VDR with ID {vdr_id} is not in dispatcher's list of provers",
            )

        with grpc.insecure_channel(self.provers[vdr_id]) as channel:
            stub = ProverStub(channel)
            proof_response = stub.get_proof(
                ProofRequest(
                    user_id=user_id,
                    age_input=self.kyc_params.minimum_age,
                    country_inputs=self.kyc_params.allowed_countries,
                )
            )
        proof = proof_response.proof

        if proof_response.status_code != 0:
            return UserResponse(
                kyc_result=False,
                status_code=1,
                status_msg=f"Problem retrieving proof from prover {self.provers[vdr_id]}: {proof_response.status_msg}",
            )

        # Step 3: Send received proof to verifier
        with grpc.insecure_channel(
            f"{self.verifier_host}:{self.verifier_port}"
        ) as channel:
            stub = VerifierStub(channel)
            kyc_result_response = stub.verify_proof(
                VerificationRequest(
                    proof=proof,
                    public_age=self.kyc_params.minimum_age,
                    public_countries=self.kyc_params.allowed_countries,
                )
            )

        kyc_result = kyc_result_response.kyc_passed
        return UserResponse(
            kyc_result=kyc_result,
            status_code=0,
            status_msg="SUCCESS",
        )
