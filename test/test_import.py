import lychpy

def test_double():
    assert lychpy.double(2) == 4

def test_sum_as_string():
    assert lychpy.sum_as_string(1, 2) == '3'


test_double()
test_sum_as_string()