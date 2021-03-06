#include <stdio.h>
#include <string.h>

#define SIZE 50

int main() {
	int open = 0, trees = 0, lumberyard = 0;
	char map[SIZE][SIZE];
	for (int y = 0; y < SIZE; ++y) {
		for (int x = 0; x < SIZE; ++x) {
			map[y][x] = getchar();
			if (map[y][x] == '.')
				++open;
			else if (map[y][x] == '|')
				++trees;
			else if (map[y][x] == '#')
				++lumberyard;
		}
		getchar(); // Skip newline
	}

	int adj[8][2] = {
		{-1, -1}, { 0, -1}, {+1, -1},
		{-1,  0},           {+1,  0},
		{-1, +1}, { 0, +1}, {+1, +1}
	};
	char temp[SIZE][SIZE];
	for (int minute = 1; minute <= 10; ++minute) {
		memcpy(&temp[0][0], &map[0][0], SIZE * SIZE * sizeof(char));
		for (int y = 0; y < SIZE; ++y) {
			for (int x = 0; x < SIZE; ++x) {
				int adjOpen = 0, adjTrees = 0, adjLumberyard = 0;
				for (int i = 0; i < 8; ++i) {
					int x2 = x + adj[i][0];
					int y2 = y + adj[i][1];
					if (x2 >= 0 && x2 < SIZE && y2 >= 0 && y2 < SIZE) {
						if (map[y2][x2] == '.')
							++adjOpen;
						else if (map[y2][x2] == '|')
							++adjTrees;
						else if (map[y2][x2] == '#')
							++adjLumberyard;
					}
				}
				if (map[y][x] == '.') {
					if (adjTrees >= 3) {
						temp[y][x] = '|';
						--open;
						++trees;
					}
				} else if (map[y][x] == '|') {
					if (adjLumberyard >= 3) {
						temp[y][x] = '#';
						--trees;
						++lumberyard;
					}
				} else if (map[y][x] == '#') {
					if (!(adjLumberyard >= 1 && adjTrees >= 1)) {
						temp[y][x] = '.';
						--lumberyard;
						++open;
					}
				}
			}
		}
		memcpy(&map[0][0], &temp[0][0], SIZE * SIZE * sizeof(char));
	}
	int result = trees * lumberyard;
	printf("Result: %d\n", result);

	return 0;
}
