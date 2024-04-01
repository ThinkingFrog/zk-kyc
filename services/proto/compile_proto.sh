#!/bin/bash

set -euo pipefail
root_dir=$(realpath $(dirname "${BASH_SOURCE[0]}")/..)

cd "$root_dir"

# Generate protobuf files
mkdir -p prover_server/proto_generated
python -m grpc_tools.protoc \
    -I proto \
    --python_out=prover_server/proto_generated \
    --grpc_python_out=prover_server/proto_generated \
    proto/prover.proto

mkdir -p verifier_server/proto_generated
python -m grpc_tools.protoc \
    -I proto \
    --python_out=verifier_server/proto_generated \
    --grpc_python_out=verifier_server/proto_generated \
    proto/verifier.proto

mkdir -p dispatcher_server/proto_generated
python -m grpc_tools.protoc \
    -I proto \
    --python_out=dispatcher_server/proto_generated \
    --grpc_python_out=dispatcher_server/proto_generated \
    proto/dispatcher.proto proto/prover.proto proto/verifier.proto

mkdir -p user/proto_generated
python -m grpc_tools.protoc \
    -I proto \
    --python_out=user/proto_generated \
    --grpc_python_out=user/proto_generated \
    proto/dispatcher.proto

# Fix python problems
touch prover_server/proto_generated/__init__.py
sed -i 's/import prover_pb2 as prover__pb2/import prover_server.proto_generated.prover_pb2 as prover__pb2/' \
    prover_server/proto_generated/prover_pb2_grpc.py

touch verifier_server/proto_generated/__init__.py
sed -i 's/import verifier_pb2 as verifier__pb2/import verifier_server.proto_generated.verifier_pb2 as verifier__pb2/' \
    verifier_server/proto_generated/verifier_pb2_grpc.py

touch dispatcher_server/proto_generated/__init__.py
sed -i 's/import dispatcher_pb2 as dispatcher__pb2/import dispatcher_server.proto_generated.dispatcher_pb2 as dispatcher__pb2/' \
    dispatcher_server/proto_generated/dispatcher_pb2_grpc.py
sed -i 's/import prover_pb2 as prover__pb2/import dispatcher_server.proto_generated.prover_pb2 as prover__pb2/' \
    dispatcher_server/proto_generated/prover_pb2_grpc.py
sed -i 's/import verifier_pb2 as verifier__pb2/import dispatcher_server.proto_generated.verifier_pb2 as verifier__pb2/' \
    dispatcher_server/proto_generated/verifier_pb2_grpc.py

touch user/proto_generated/__init__.py
sed -i 's/import dispatcher_pb2 as dispatcher__pb2/import user.proto_generated.dispatcher_pb2 as dispatcher__pb2/' \
    user/proto_generated/dispatcher_pb2_grpc.py

# Run formatting
pre-commit run --files prover_server/proto_generated/*
pre-commit run --files verifier_server/proto_generated/*
pre-commit run --files dispatcher_server/proto_generated/*
pre-commit run --files user/proto_generated/*
