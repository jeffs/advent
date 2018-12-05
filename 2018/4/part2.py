#!/usr/bin/env python3


from collections import namedtuple
from operator import itemgetter


def parse_record(line):
    parts = line.split()
    hour, minute = map(int, parts[1][:-1].split(':'))
    return hour, minute, parts[3] # guard ID, 'asleep', or 'up'


assert parse_record('[1518-05-18 00:35] falls asleep\n') == (0, 35, 'asleep')
assert parse_record('[1518-09-27 00:40] wakes up\n') == (0, 40, 'up')
assert parse_record('[1518-04-10 23:52] Guard #3559 begins shift\n') == (23, 52, '#3559')


def parse_lines(lines):
    nap_dict = {}   # maps guard IDs to lists of naps
    if lines:
        guard_id = None
        for hour, minute, tag in map(parse_record, lines):
            if tag.startswith('#'):
                guard_id = int(tag[1:])
            elif tag == 'asleep':
                nap_begin = (hour, minute)
            elif tag == 'up':
                nap_end = (hour, minute)
                naps = nap_dict.setdefault(guard_id, [])
                naps.append((nap_begin, nap_end))
            else:
                raise 'bad tag: %'.format(tag)
    return nap_dict


assert parse_lines((
    '[1518-11-01 00:00] Guard #10 begins shift\n',
    '[1518-11-01 00:05] falls asleep\n',
    '[1518-11-01 00:25] wakes up\n',
    '[1518-11-01 00:30] falls asleep\n',
    '[1518-11-01 00:55] wakes up\n',
)) == {
    10: [
        ((0,  5), (0, 25)),
        ((0, 30), (0, 55))
    ],
}


def read_records(fname):
    with open('input') as stream:
        lines = stream.readlines()
    lines.sort()
    return parse_lines(lines)

def get_nap_length(nap):
    """Return the length of the specified nap in minutes."""
    begin, end = nap
    begin_hour, begin_minute = begin
    end_hour, end_minute = end
    if end < begin:
        end_hour += 24
    return (end_hour * 60 + end_minute) - (begin_hour * 60 + begin_minute)


def next_time(time):
    hour, minute = time
    minute = (minute + 1) % 60
    if minute == 0:
        hour = (hour + 1) % 24
    return hour, minute


def count_minutes(naps):
    counts = {}
    for nap in naps:
        time, end = nap
        while time != end:
            counts[time] = counts[time] + 1 if time in counts else 1
            time = next_time(time)
    return counts


def max_by_value(dictionary):
    return max(dictionary.items(), key=itemgetter(1))[0]


if __name__ == '__main__':

    nap_dict = read_records('input')    # guard -> naps

    max_guard_id, max_time, max_count = None, None, None
    for guard_id, naps in nap_dict.items():
        counts = count_minutes(naps)
        time = max_by_value(counts)
        count = counts[time]
        if max_guard_id == None or count > max_count:
            max_guard_id, max_time, max_count = guard_id, time, count

    max_minute = max_time[1]
    print(max_guard_id * max_minute)
