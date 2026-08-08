import importlib
import sys
from pathlib import Path

from fastapi.testclient import TestClient


def _load_main(monkeypatch):
    # Set required env var before importing module (module reads it at import time)
    monkeypatch.setenv("ORGANIZATION_MANAGENEBT_PORT", "12345")
    repo_root = Path(__file__).resolve().parents[4]

    # Ensure the repository root is on sys.path so the packages.* import works
    sys.path.insert(0, str(repo_root))

    module_name = "packages.subgraphs.organization_management.main"
    # Remove any previously loaded module to ensure a fresh import
    for key in (module_name, "organization_management_main", "organization_management"):
        sys.modules.pop(key, None)

    module = importlib.import_module(module_name)
    # Reload to be sure the environment variable set above is picked up
    importlib.reload(module)
    return module


def test_user_query_returns_expected(monkeypatch):
    module = _load_main(monkeypatch)
    client = TestClient(module.app)
    resp = client.post("/graphql", json={"query": "{ user { id name } }"})
    assert resp.status_code == 200
    assert resp.json() == {"data": {"user": {"id": "1", "name": "Patrick"}}}


def test_graphql_endpoint_mounts_and_returns_json(monkeypatch):
    module = _load_main(monkeypatch)
    client = TestClient(module.app)
    resp = client.post("/graphql", json={"query": "{ user { id name } }"})
    assert resp.headers["content-type"].startswith("application/json")
    payload = resp.json()
    assert "data" in payload and "user" in payload["data"]


def test_schema_execute_directly(monkeypatch):
    module = _load_main(monkeypatch)
    result = module.schema.execute_sync("{ user { id name } }")
    assert not result.errors
    assert result.data["user"] == {"id": "1", "name": "Patrick"}
