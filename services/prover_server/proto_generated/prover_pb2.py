# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: prover.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\x0cprover.proto\x12\x06zk_kyc"J\n\x0cProofRequest\x12\x0f\n\x07user_id\x18\x01 \x01(\x04\x12\x11\n\tage_input\x18\x02 \x01(\x04\x12\x16\n\x0e\x63ountry_inputs\x18\x03 \x03(\x04"G\n\rProofResponse\x12\r\n\x05proof\x18\x01 \x01(\x0c\x12\x13\n\x0bstatus_code\x18\x02 \x01(\r\x12\x12\n\nstatus_msg\x18\x03 \x01(\t2D\n\x06Prover\x12:\n\tget_proof\x12\x14.zk_kyc.ProofRequest\x1a\x15.zk_kyc.ProofResponse"\x00\x62\x06proto3'
)

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, "prover_pb2", globals())
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    _PROOFREQUEST._serialized_start = 24
    _PROOFREQUEST._serialized_end = 98
    _PROOFRESPONSE._serialized_start = 100
    _PROOFRESPONSE._serialized_end = 171
    _PROVER._serialized_start = 173
    _PROVER._serialized_end = 241
# @@protoc_insertion_point(module_scope)