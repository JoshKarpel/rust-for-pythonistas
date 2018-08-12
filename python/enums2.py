from enum import Enum


class Direction(Enum):
    Up = 'up'
    Down = 'down'
    Left = 'left'
    Right = 'right'

    def to_arrow(self) -> str:
        return type_to_arrow[self]


type_to_arrow = {
    Direction.Up: '↑',
    Direction.Down: '↓',
    Direction.Left: '←',
    Direction.Right: '→',
}

if __name__ == '__main__':
    for direction in Direction:
        print(direction.to_arrow())
