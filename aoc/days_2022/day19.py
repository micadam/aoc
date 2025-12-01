from dataclasses import dataclass
from functools import cache
import math

from aoc.day import Day


@dataclass
class Blueprint:
    ore_robot_ore: int
    clay_robot_ore: int
    obsidian_robot_ore: int
    obsidian_robot_clay: int
    geode_robot_ore: int
    geode_robot_obs: int

    def __hash__(self) -> int:
        return hash((self.ore_robot_ore, self.clay_robot_ore,
                     self.obsidian_robot_ore, self.obsidian_robot_clay,
                     self.geode_robot_ore, self.geode_robot_obs))


class Day19(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(19, test)
        self.blueprints = []
        for line in self.lines:
            segments = line.split()
            ore_robot = int(segments[6])
            clay_robot = int(segments[12])
            obs_robot_ore = int(segments[18])
            obs_robot_clay = int(segments[21])
            geode_robot_ore = int(segments[27])
            geode_robot_obs = int(segments[30])
            self.blueprints.append(Blueprint(ore_robot, clay_robot,
                                             obs_robot_ore, obs_robot_clay,
                                             geode_robot_ore, geode_robot_obs
                                             ))

    def part1(self):
        ans = 0
        for i, blueprint in enumerate(self.blueprints):
            self.__get_max_geode.cache_clear()
            ans += (i + 1) * self.__get_max_geode(24, blueprint)
        return ans

    def part2(self):
        ans = 1
        for blueprint in self.blueprints[:3]:
            self.__get_max_geode.cache_clear()
            ans *= self.__get_max_geode(32, blueprint)
        return ans

    @cache
    def __get_max_geode(self, T: int, blueprint: Blueprint,
                        ore: int = 0, clay: int = 0, obs: int = 0,
                        ore_r: int = 1, clay_r: int = 0, obs_r: int = 0):
        if not T:
            return 0

        ans = 0
        # build ore
        dT = T + 1
        if ore_r > 0:
            dT = max(0, math.ceil((blueprint.ore_robot_ore - ore) / ore_r))
        if ore_r < max(blueprint.ore_robot_ore,
                       blueprint.clay_robot_ore,
                       blueprint.obsidian_robot_ore,
                       blueprint.geode_robot_ore) and T > dT:
            # spend one turn building
            dT += 1
            new_ore = ore - blueprint.ore_robot_ore + dT * ore_r
            ans = max(ans, self.__get_max_geode(
                T - dT, blueprint,
                new_ore, clay + dT * clay_r, obs + dT * obs_r,
                ore_r + 1, clay_r, obs_r))
        # build clay
        dT = T + 1
        if ore_r > 0:
            dT = max(0, math.ceil((blueprint.clay_robot_ore - ore) / ore_r))
        if clay_r < blueprint.obsidian_robot_clay and T > dT:
            dT += 1
            new_ore = ore - blueprint.clay_robot_ore + dT * ore_r
            ans = max(ans, self.__get_max_geode(
                T - dT, blueprint,
                new_ore, clay + dT * clay_r, obs + dT * obs_r,
                ore_r, clay_r + 1, obs_r))
        # build obs
        dT = T + 1
        if ore_r > 0 and clay_r > 0:
            dT = max(0, max(
                math.ceil((blueprint.obsidian_robot_ore - ore) / ore_r),
                math.ceil((blueprint.obsidian_robot_clay - clay) / clay_r)))
        if obs_r < blueprint.geode_robot_obs and T > dT:
            dT += 1
            new_ore = ore - blueprint.obsidian_robot_ore + dT * ore_r
            new_clay = clay - blueprint.obsidian_robot_clay + dT * clay_r
            ans = max(ans, self.__get_max_geode(
                T - dT, blueprint,
                new_ore, new_clay, obs + dT * obs_r,
                ore_r, clay_r, obs_r + 1))
        # build geode
        dT = T + 1
        if ore_r > 0 and obs_r > 0:
            dT = max(0, max(
                math.ceil((blueprint.geode_robot_ore - ore) / ore_r),
                math.ceil((blueprint.geode_robot_obs - obs) / obs_r)))
        if T > dT:
            dT += 1
            new_ore = ore - blueprint.geode_robot_ore + dT * ore_r
            new_obs = obs - blueprint.geode_robot_obs + dT * obs_r
            ans = max(ans, (T - dT) + self.__get_max_geode(
                T - dT, blueprint,
                new_ore, clay + dT * clay_r, new_obs,
                ore_r, clay_r, obs_r))
        return ans
