import random
import turtle
import colorsys

radius = 100
t = turtle.Turtle()
t.speed(100)
t.penup()

def rect(width, height, flip):
    rgb = colorsys.hls_to_rgb(random.random(), 50 / 100, 100 / 100)
    color = "#" + "%02X" % int(rgb[0] * 255) + "%02X" % int(rgb[1] * 255) + "%02X" % int(rgb[2] * 255)
    t.fillcolor(color)
    t.begin_fill()
    head = 0
    if flip:
        head = 0
    else:
        head = 90
    t.setheading(head)
    t.forward(width)
    t.right(90)
    t.forward(height)
    t.right(90)
    t.forward(width)
    t.right(90)
    t.forward(height)
    t.end_fill()
    t.setheading(head - 90)
    t.forward(height)

def main():
    stepcount = int(input("Zadaj pocet schodov: "))

    initial = t.pos()

    for i in range(0, stepcount):
        print(" " * 4 * i + "*****")

    for i in range(0, stepcount):
        j = stepcount - i - 1
        rect(j * 10, 10, False)

    t.setpos(initial)

    t.setheading(180)
    t.forward(200)
    t.setheading(0)

    for i in range(0, stepcount):
        j = stepcount - i - 1
        rect(j* 10, 10, True)

    input()

if __name__ == "__main__":
    main()