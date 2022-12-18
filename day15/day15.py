from collections import namedtuple
from pathlib import Path

from parse import parse

TEST_INPUT = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")  # 8450957848888 too low


def test_part_1():
    assert part_1(TEST_INPUT) == 26


def test_part_2():
    assert part_2(TEST_INPUT) == 56000011


Sensor = namedtuple("Sensor", ["x", "y", "distance"])
Beacon = namedtuple("Beacon", ["x", "y"])


def overlaps(this_range, other_range):
    """assuming sorted ranges"""
    return other_range[1] >= this_range[0] and other_range[0] <= this_range[1]


def part_1(input_str):
    sensors = []
    x_min = 1e10
    x_max = -1e10
    y_min = 1e10
    y_max = -1e10
    for line in input_str.splitlines():
        result = parse("Sensor at x={sensor.x:d}, y={sensor.y:d}: closest beacon is at x={beacon.x:d}, y={beacon.y:d}",
                       line)

        beacon = Beacon(result["beacon.x"], result["beacon.y"])
        distance = abs(result["sensor.x"] - beacon.x) + \
                   abs(result["sensor.y"] - beacon.y
                       )
        sensor = Sensor(result["sensor.x"], result["sensor.y"], distance=distance)
        sensors.append(sensor)
        x_min = min(x_min, beacon.x, sensor.x)
        x_max = max(x_max, beacon.x, sensor.x)
        y_min = min(y_min, beacon.y, sensor.y)
        y_max = max(y_max, beacon.y, sensor.y)
    target_row = 2_000_000 if y_max > 1000 else 10

    def coverage(row, sensors, x_min, x_max):
        sensors.sort(key=lambda s: s.x)
        ranges = []
        for sensor in sensors:
            sensor_range = [sensor.x - (sensor.distance - abs(sensor.y - row)),
                            sensor.x + (sensor.distance - abs(sensor.y - row))
                            ]
            try:
                previous_range = ranges.pop()
            except IndexError:
                ranges.append(sensor_range)
                continue
            if overlaps(sensor_range, previous_range):
                current_range = [min(sensor_range[0], previous_range[0]),
                                 max(sensor_range[1], previous_range[1])]
                ranges.append(current_range)
            else:
                ranges.append(previous_range)
                ranges.append(sensor_range)
        covered_positions = 0
        for range_ in ranges:
            covered_positions += min(range_[1], x_max) - max(range_[0], x_min)
        return covered_positions

    return coverage(row=target_row,
                    sensors=[s for s in sensors
                             if abs(s.y - target_row) <= s.distance
                             ],
                    x_min=x_min,
                    x_max=x_max
                    )


def overlaps_or_touches(this_range, previous_range):
    """assuming sorted ranges"""
    return previous_range[1] + 1 > this_range[0]


def part_2(input_str):
    sensors = []
    x_min = 1e10
    x_max = -1e10
    y_min = 1e10
    y_max = -1e10
    for line in input_str.splitlines():
        result = parse("Sensor at x={sensor.x:d}, y={sensor.y:d}: closest beacon is at x={beacon.x:d}, y={beacon.y:d}",
                       line)

        beacon = Beacon(result["beacon.x"], result["beacon.y"])
        distance = abs(result["sensor.x"] - beacon.x) + abs(result["sensor.y"] - beacon.y)
        sensor = Sensor(result["sensor.x"], result["sensor.y"], distance=distance)
        sensors.append(sensor)
        x_min = min(x_min, beacon.x, sensor.x)
        x_max = max(x_max, beacon.x, sensor.x)
        y_min = min(y_min, beacon.y, sensor.y)
        y_max = max(y_max, beacon.y, sensor.y)
    max_row = 4_000_000 if y_max > 1000 else 20

    def freq(row, sensors):
        sensors.sort(key=lambda s: s.x - (s.distance - abs(s.y - row)))
        ranges = []
        for sensor in sensors:
            sensor_range = [sensor.x - (sensor.distance - abs(sensor.y - row)),
                            sensor.x + (sensor.distance - abs(sensor.y - row))
                            ]
            try:
                previous_range = ranges.pop()
            except IndexError:
                ranges.append(sensor_range)
                continue
            if overlaps_or_touches(sensor_range, previous_range):
                current_range = [min(sensor_range[0], previous_range[0]),
                                 max(sensor_range[1], previous_range[1])]
                ranges.append(current_range)
            else:
                if sensor_range[0] - previous_range[1] > 1 \
                        and sensor_range[0] <= max_row:
                    return (sensor_range[0] - 1) * 4_000_000 + row
        return None

    for row in range(max_row):
        frequency = freq(row=row,
                         sensors=[s for s in sensors if abs(s.y - row) <= s.distance],
                         )
        if frequency:
            return frequency


if __name__ == "__main__":
    main()
