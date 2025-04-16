import turtle

t = turtle.Turtle()

def main():
    print()
    side = int(input("strana? "))
    n = int(input("kolko uholnik? "))
    x = int(input("kolko opakovani? "))
    angle = 360/x
    for i in range(0, x):
        t.setheading(i * angle)
        nuholnik(n, side)

    input()


def nuholnik(n: int, side: int):
    angle = 360 / n
    for i in range(0, n):
        t.forward(side)
        t.right(angle)


if __name__ == "__main__":
    main()