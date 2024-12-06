import turtle

t = turtle.Turtle()
t.speed(100)
t.penup()

def a():
    t.dot(100, "red")
    t.dot(85, "white")
    t.setheading(-135)
    t.forward(50)
    t.write("80", font=("Arial", 48))

def b():
    t.dot(100, "red")
    t.dot(85, "white")
    t.setheading(-135)
    t.forward(50)
    t.write("6 t", font=("Arial", 48))

def main():
    func = input("a or b\n")
    globals().get(func)()

    input()

if __name__ == "__main__":
    main()