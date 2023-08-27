import pylychee

def test_double():
    assert pylychee.double(2) == 4

def test_sum_as_string():
    assert pylychee.sum_as_string(1, 2) == '3'


test_double()
test_sum_as_string()