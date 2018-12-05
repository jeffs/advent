#!/usr/bin/env python3

def hamming(string_a, string_b):
    """
    Return the Hamming Distance between the specified strings.  In other words:
    Return the number of positions at which string_a and string_b hold
    different characters.  The strings must be of the same length.
    """
    assert len(string_a) == len(string_b)
    return sum(1 for a, b in zip(string_a, string_b) if a != b)

def find_hamming_1(box_ids):
    """
    Return a pair of box_ids whose Hamming Distance is 1.  Return None if no
    such pair is found.  The box_ids parameter must be an indexed type (e.g., a
    list).
    """
    for i in range(len(box_ids) - 1):
        string_a = box_ids[i]
        for j in range(i + 1, len(box_ids)):
            string_b = box_ids[j]
            if hamming(string_a, string_b) == 1:
                return string_a, string_b


with open('input') as stream:
    box_ids = tuple(stream)

string_a, string_b = find_hamming_1(box_ids)
print(''.join(a for a, b in zip(string_a, string_b) if a == b))
