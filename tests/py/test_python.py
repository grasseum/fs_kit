import _fskit_lib


def test_get_validate_dir_empty_input():
    assert _fskit_lib.glob([]) == []


def test_get_validate_dir_finds_subfolders(tmp_path, monkeypatch):
    (tmp_path / "alpha").mkdir()
    (tmp_path / "beta").mkdir()
    monkeypatch.chdir(tmp_path)

    result = _fskit_lib.glob(["./**"])

    assert {entry.name for entry in result} == {"alpha", "beta"}
    assert all(entry.info == "folder" for entry in result)


def test_glob_accepts_string_and_list(tmp_path, monkeypatch):
    (tmp_path / "alpha").mkdir()
    (tmp_path / "beta").mkdir()
    monkeypatch.chdir(tmp_path)

    expected = {"alpha", "beta"}
    assert {entry.name for entry in _fskit_lib.glob("./**")} == expected
    assert {entry.name for entry in _fskit_lib.glob(["./**"])} == expected
