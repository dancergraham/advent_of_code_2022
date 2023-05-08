from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path

TEST_INPUT = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""


@dataclass
class Valve:
    name: str
    flow_rate: int
    neighbours: list


def parse_input(input_str):
    valves = []
    for line in input_str.splitlines():
        name = line[6:8]
        flow_rate = int(line.split("=")[1].split(";")[0])
        neighbours = line.split("valve")[-1].removeprefix("s").strip().split(", ")
        valves.append(Valve(name, flow_rate, neighbours))
    return valves


def main():
    print(f"{part_1(Path('input.txt').read_text())}")
    print(f"{part_2(Path('input.txt').read_text())}")


def test_part_1():
    assert part_1(TEST_INPUT) == 1651


def test_part_2():
    assert part_2(TEST_INPUT) == 1707


def part_1(input_str):
    def sortup(tup):
        return tuple(sorted(tup))
    @lru_cache(maxsize=100_000_000)
    def maximum_pressure_release(current_valve="AA",
                                 previous_valve="",
                                 opened_valves=(),
                                 minute=0,
                                 ):
        minute += 1
        if minute == 30:
            pressure_release = 0
        else:
            options = []
            if current_valve not in opened_valves and valves[current_valve].flow_rate:
                options.append(maximum_pressure_release(current_valve=current_valve,
                                                        previous_valve=previous_valve,
                                                        opened_valves=sortup(opened_valves + (current_valve,)),
                                                        minute=minute)
                               )
            options.extend(maximum_pressure_release(current_valve=neighbour,
                                                    previous_valve=current_valve,
                                                    opened_valves=opened_valves,
                                                    minute=minute)
                           for neighbour in valves[current_valve].neighbours
                           )
            pressure_release = max(options)
        pressure_release += sum(valves[valve].flow_rate for valve in valves if valve in opened_valves)

        return pressure_release

    valves = {valve.name: valve for valve in parse_input(input_str)}

    return maximum_pressure_release()


def part_2(input_str):
    def sortup(tup):
        return tuple(sorted(tup))
    @lru_cache(maxsize=100_000_000)
    def maximum_pressure_release(current_valve_me="AA",
                                 current_valve_elephant="AA",
                                 previous_valve="",
                                 previous_valve_elephant="AA",
                                 opened_valves=(),
                                 minute=0,
                                 ):
        minute += 1
        if minute == 30:
            pressure_release = 0
        else:
            options_me = []
            if current_valve_me not in opened_valves and valves[current_valve_me].flow_rate:
                options_me.append(maximum_pressure_release(current_valve_me=current_valve_me,
                                                        previous_valve=previous_valve,
                                                        opened_valves=sortup(opened_valves + (current_valve_me,)),
                                                        minute=minute)
                               )
            options_me.extend(maximum_pressure_release(current_valve_me=neighbour,
                                                    previous_valve=current_valve_me,
                                                    opened_valves=opened_valves,
                                                    minute=minute)
                           for neighbour in valves[current_valve_me].neighbours
                           )
            pressure_release = max(options_me)
        pressure_release += sum(valves[valve].flow_rate for valve in valves if valve in opened_valves)

        return pressure_release

    valves = {valve.name: valve for valve in parse_input(input_str)}

    return maximum_pressure_release()



if __name__ == '__main__':
    main()