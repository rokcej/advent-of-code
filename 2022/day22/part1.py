DIRS = [(0, +1), (+1, 0), (0, -1), (-1, 0)]

def get_path(line: str) -> list:
	path = []
	i = 0
	while i < len(line):
		if line[i].isalpha():
			path.append(line[i])
			i += 1
		else:
			j = i + 1
			while j < len(line) and line[j].isnumeric():
				j += 1
			path.append(int(line[i:j]))
			i = j
	return path

def wrap(val, min_val, max_val):
	if val > max_val:
		return min_val
	if val < min_val:
		return max_val
	return val


with open("input", "r") as f:
	lines = f.read().splitlines()

i_empty = lines.index("")
board = lines[:i_empty]
path = get_path(lines[i_empty+1])

y, x = 0, board[0].find(".")
dir = 0

h = len(board)
w = max(len(row) for row in board)

# Get index ranges for each row
row_lims = []
for r in range(h):
	start = -1
	for c in range(len(board[r])):
		if board[r][c] != " ":
			start = c
			break
	row_lims.append((start, len(board[r]) - 1))

# Get index ranges for each column
col_lims = []
for c in range(w):
	start = 0
	for r in range(h):
		if row_lims[r][0] <= c <= row_lims[r][1]:
			start = r
			break
	stop = h - 1
	for r in range(start + 1, h):
		if not (row_lims[r][0] <= c <= row_lims[r][1]):
			stop = r - 1
			break
	col_lims.append((start, stop))

for element in path:
	# Change direction
	if element == "R":
		dir = (dir + 1) % 4
		continue
	if element == "L":
		dir = (dir + 3) % 4
		continue
	
	# Move
	distance = element
	for _ in range(distance):
		dy, dx = DIRS[dir]
		y2, x2 = y + dy, x + dx
		if dy == 0:
			x2 = wrap(x2, *row_lims[y2])
		else:
			y2 = wrap(y2, *col_lims[x2])

		if board[y2][x2] == "#":
			break
		y, x = y2, x2

print(1000 * (y + 1) + 4 * (x + 1) + dir)
