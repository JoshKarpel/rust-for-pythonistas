from enum import Enum


class Direction(Enum):
    Up = 'up'
    Down = 'down'
    Left = 'left'
    Right = 'right'

    def to_arrow(self) -> str:
        return type_to_arrow[self.value]


type_to_arrow = {
    Direction.Up.value: '↑',
    Direction.Down.value: '↓',
    Direction.Left.value: '←',
    Direction.Right.value: '→',
}

if __name__ == '__main__':
    for direction in Direction:
        print(direction.to_arrow())
