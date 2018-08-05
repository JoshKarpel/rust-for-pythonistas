class Direction:
    pass


class Up(Direction):
    pass


class Down(Direction):
    pass


class Left(Direction):
    pass


class Right(Direction):
    pass


type_to_arrow = {
    Up: '↑',
    Down: '↓',
    Left: '←',
    Right: '→',
}


def to_arrow(dir: Direction) -> str:
    return type_to_arrow[type(dir)]


if __name__ == '__main__':
    print(to_arrow(Up()))
    print(to_arrow(Down()))
    print(to_arrow(Left()))
    print(to_arrow(Right()))
