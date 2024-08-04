import sys
from random import shuffle, randrange

def make_maze(w=16, h=8):
    vis = [[0] * w + [1] for _ in range(h)] + [[1] * (w + 1)]
    ver = [["|  "] * w + ['|'] for _ in range(h)] + [[]]
    hor = [["+--"] * w + ['+'] for _ in range(h + 1)]

    def walk(x, y):
        vis[y][x] = 1

        d = [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
        shuffle(d)
        for (xx, yy) in d:
            if vis[yy][xx]: continue
            if xx == x: hor[max(y, yy)][x] = "+  "
            if yy == y: ver[y][max(x, xx)] = "   "
            walk(xx, yy)

    walk(randrange(w), randrange(h))

    s = ""
    for (a, b) in zip(hor, ver):
        s += ''.join(a + ['\n'] + b + ['\n'])
    
    l = list(s)
    l[w * 3 + 3] = 'p'
    l[((w * 3 + 3) * -1) - 3] = 'g'
    return "".join(l)

if __name__ == '__main__':
    w = 10  
    h = 10  
    if len(sys.argv) > 2:
        w = int(sys.argv[1])
        h = int(sys.argv[2])

    maze = make_maze(w, h)
    with open("maze.txt", "w") as f:
        f.write(maze)
