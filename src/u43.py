import turtle

t = turtle.Turtle()

def main():
    t.penup()
    t.setheading(0)
    for i in range(0, 30):
        next(i)
    input()

def next(i):
    t.goto(30 * i, 0)
    color = ""
    if i % 3 == 1:
        color = "yellow"
    elif i % 3 == 2:
        color = "red"
    elif i % 6 == 0:
        color = "black"
    elif i % 6 == 3:
        color = "blue"
    t.dot(30, color)

if __name__ == "__main__":
    main()