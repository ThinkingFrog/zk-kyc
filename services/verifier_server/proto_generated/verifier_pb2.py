# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: verifier.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\x0everifier.proto\x12\x06zk_kyc"R\n\x13VerificationRequest\x12\r\n\x05proof\x18\x01 \x01(\x0c\x12\x12\n\npublic_age\x18\x02 \x01(\x04\x12\x18\n\x10public_countries\x18\x03 \x03(\x04"*\n\x14VerificationResponse\x12\x12\n\nkyc_passed\x18\x01 \x01(\x08\x32W\n\x08Verifier\x12K\n\x0cverify_proof\x12\x1b.zk_kyc.VerificationRequest\x1a\x1c.zk_kyc.VerificationResponse"\x00\x62\x06proto3'
)

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, "verifier_pb2", globals())
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    _VERIFICATIONREQUEST._serialized_start = 26
    _VERIFICATIONREQUEST._serialized_end = 108
    _VERIFICATIONRESPONSE._serialized_start = 110
    _VERIFICATIONRESPONSE._serialized_end = 152
    _VERIFIER._serialized_start = 154
    _VERIFIER._serialized_end = 241
# @@protoc_insertion_point(module_scope)