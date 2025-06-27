"""Some fixtures for the tests."""

import random

import pytest

MAX_LEN = 1000
STEP = 100
MIN = -100
MAX = 100
PARAMS = list(range(STEP, MAX_LEN + STEP, STEP))
IDS = [f"len={len}" for len in PARAMS]


@pytest.fixture(params=PARAMS, ids=IDS)
def gen_list(request: pytest.FixtureRequest) -> list[int]:
    """Generate random data."""
    len: int = request.param

    return [random.randint(MIN, MAX) for _ in range(len)]
