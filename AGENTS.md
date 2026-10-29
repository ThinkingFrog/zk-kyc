# AGENTS.md

ZK-KYC: ZKP-based KYC system. Bachelor thesis project. Two independent parts:

- `zk_backend/` — Rust workspace: `zkkyc_lib` (halo2 circuit, proving/verifying API) and `zkkyc_c_bindings` (cdylib exposing the API over C ABI, loaded from Python via `ctypes`). Docs (README, DESCRIPTION.md) are in Russian.
- `services/` — Python 3.10 gRPC microservices: `prover_server`, `verifier_server`, `dispatcher_server`, `user` (Flask web UI), `vdr` (ClickHouse data store + one-shot DB filler), `db_connector`, `proto`, `config`, `lib`.

## Running the system

All service commands run from `services/`:

1. `./lib/build_lib.sh` — builds the Rust lib inside Docker (rust:1.68.0) and copies `libzkkyc_c_bindings.so` into `services/lib/` (gitignored). **Required before first `docker compose up`**: compose bind-mounts `services/lib/libzkkyc_c_bindings.so` into prover/verifier containers; startup fails without it.
2. `docker compose up -d` — starts everything. Use `--build` after changing service code.
3. Web UI (submits KYC requests): http://localhost:5000 (`user` service).

All services except ClickHouse use `network_mode: host`. Ports: 5000 web UI, 50051 prover, 50052 dispatcher, 50053 verifier, 18123/19000 ClickHouse (HTTP/native).

Configs in `services/config/` (TOML, one per service; bind-mounted). **Not hot-reloaded — restart the container to apply config changes.** `config/*.local*` is the gitignore pattern for local overrides. KYC constraints live in `config/kyc_params.json` (minimum_age, allowed countries) — the dispatcher passes them to prover and verifier as public inputs.

`vdr-filler` is a one-shot container that loads `vdr/db_test_data.csv` into ClickHouse table `test_dataset` (USER_ID, AGE, COUNTRY_ID) after the server healthcheck passes, then exits. The prover queries this table directly.

## Protobuf

gRPC protos live in `services/proto/`. Regenerate with `./proto/compile_proto.sh` from `services/` — requires `grpcio-tools` and `pre-commit` (with black/isort) installed locally. The script generates into each service's `proto_generated/` dir and then `sed`-fixes the imports to absolute package paths. Never hand-edit `proto_generated/`; regenerate instead. Generated files are committed.

## Rust backend

- Workspace: `zk_backend/`, release profile uses `strip`, `lto`, `opt-level = "z"`, `panic = "abort"` (test with the default profile, not release).
- Tests: `cargo test` from `zk_backend/` (or single: `cargo test -p zkkyc_lib lib_api_simple`). The Dockerfile also has a `test` stage before the build stage.
- `zkkyc_c_bindings/build.rs` runs cbindgen to emit `zkkyc_c_bindings/zkkyc_lib.h`; the C header is regenerated on every build.
- Circuit: `zkkyc_lib/src/core/circuit.rs` — proves age >= minimum_age and country in the allowed list; `K = 4` (params size); halo2 0.2 with pasta `Fp`.

## Python services

- No pyproject/uv setup; services run in Docker containers (per-service `requirements.txt`, pinned Python 3.10.8). Running locally is not the supported path — use containers, or `python -m <service> --config config/<service>.toml` from `services/` with your own venv if needed.
- No Python tests exist. Formatting: black + isort (profile=black), wired through pre-commit (`.pre-commit-config.yaml`); pre-commit also runs JSON/TOML/whitespace checks.
- Entry pattern: each service is a `click` CLI reading a TOML config via `click-config-file`, real logic in `<service>/core/`.
- The prover and verifier load the same `.so` via `ctypes.CDLL` and declare argtypes/restype manually — keep C signatures in `zkkyc_c_bindings/src/api/` in sync with the Python `ctypes` declarations in `services/prover_server/core/service.py` and `services/verifier_server/core/service.py`.

## Notes

- No CI; verification is manual (Rust tests + bring-up of compose stack).
- Images in `docs/` use Git LFS (see `.gitattributes`).
- `services/lib/` is fully gitignored except `build_lib.sh` — never commit build artifacts.
