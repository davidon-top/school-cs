import turtle

radius = 100
t = turtle.Turtle()
t.speed(100)
t.penup()

def a():
    t.setheading(180)
    t.forward(radius*3)
    circle2(False)
    t.setheading(0)
    t.penup()
    t.forward(radius*2)
    circle2(True)
    t.setheading(0)
    t.penup()
    t.forward(radius*2)
    circle2(False)
    t.setheading(0)
    t.penup()
    t.forward(radius*2)
    circle2(True)

def b():
    circle2(False)
    t.setheading(90)
    t.forward(radius*2)
    circle2(True)
    t.setheading(180)
    t.forward(radius*2)
    circle2(False)
    t.setheading(270)
    t.forward(radius*2)
    circle2(True)


def c():
    circle2(False)
    t.setheading(90)
    t.forward(radius*2)
    circle2(False)
    t.setheading(180)
    t.forward(radius*2)
    circle2(True)
    t.setheading(270)
    t.forward(radius*2)
    circle2(True)

def circle2(reverse):
    if reverse:
        t.dot(radius*2, "black")
        t.dot(radius, "gray")
    else:
        t.dot(radius*2, "gray")
        t.dot(radius, "black")

def main():
    func = input("a b abo c\n")
    globals().get(func)()

    input()

if __name__ == "__main__":
    main()