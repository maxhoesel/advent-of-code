from day02_parser import CubeColor, Cubes, CubeSet, Game, parse_games

from .inputs import TEST, INPUT

BAG_REDS = 12
BAG_GREENS = 13
BAG_BLUES = 14


def main():
    test_games = parse_games(TEST)
    input_games = parse_games(INPUT)

    print(f"Test valid game sum: {valid_id_sum(test_games)}")
    print(f"Main valid game sum: {valid_id_sum(input_games)}")

    print(f"Test total game power: {game_power_sum(test_games)}")
    print(f"Main total game power: {game_power_sum(input_games)}")


def valid_id_sum(games: list[Game]) -> int:
    total = 0
    for game in games:
        total += check_game(game)
    return total


def game_power_sum(games: list[Game]) -> int:
    total = 0
    for game in games:
        total += game_power(game)
    return total


def game_power(game: Game) -> int:
    min_red = 0
    min_blue = 0
    min_green = 0
    for play in game.shown:
        for cubes in play.cubes:
            if cubes.color == CubeColor.BLUE:
                min_blue = max(min_blue, cubes.amount)
            elif cubes.color == CubeColor.RED:
                min_red = max(min_red, cubes.amount)
            if cubes.color == CubeColor.GREEN:
                min_green = max(min_green, cubes.amount)
    return min_blue * min_green * min_red


def check_game(game: Game) -> int:
    for play in game.shown:
        for cubes in play.cubes:
            if cubes.color == CubeColor.BLUE and cubes.amount > BAG_BLUES:
                return 0
            elif cubes.color == CubeColor.RED and cubes.amount > BAG_REDS:
                return 0
            elif cubes.color == CubeColor.GREEN and cubes.amount > BAG_GREENS:
                return 0
    return game.num
