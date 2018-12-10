#include <stdio.h>
#include <string.h>
#include <time.h>

#define LEN 379

typedef struct _Point {
	int x;
	int y;
	int dx;
	int dy;
} Point;


void getMinMax(Point *points, int *xMin, int *xMax, int *yMin, int *yMax) {
	for (int i = 0; i < LEN; ++i) {		
		if (i == 0) {
			*xMin = points[i].x;
			*xMax = points[i].x;

			*yMin = points[i].y;
			*yMax = points[i].y;
		} else {
			if (points[i].x < *xMin)
				*xMin = points[i].x;
			else if (points[i].x > *xMax)
				*xMax = points[i].x;

			if (points[i].y < *yMin)
				*yMin = points[i].y;
			else if (points[i].y > *yMax)
				*yMax = points[i].y;
		}
	}
}

int main() {
	Point points[LEN];
	int xMin, xMax, yMin, yMax;
	// Read data and get min max
	for (int i = 0; i < LEN; ++i)
		scanf("position=<%d, %d> velocity=<%d, %d>\n", &(points[i].x), &(points[i].y), &(points[i].dx), &(points[i].dy));
	getMinMax(points, &xMin, &xMax, &yMin, &yMax);

	// Simulate until area stops getting smaller
	int seconds = 0;
	long long areaPrev;
	do {
		areaPrev = (long long)(xMax - xMin) * (long long)(yMax - yMin);
		// Go 1 step forward
		for (int i = 0; i < LEN; ++i) {
			points[i].x += points[i].dx;
			points[i].y += points[i].dy;
		}
		getMinMax(points, &xMin, &xMax, &yMin, &yMax);
		seconds++;
	} while ((long long)(xMax - xMin) * (long long)(yMax - yMin) < areaPrev);

	// Go 1 step back
	for (int i = 0; i < LEN; ++i) {
		points[i].x -= points[i].dx;
		points[i].y -= points[i].dy;
	}
	getMinMax(points, &xMin, &xMax, &yMin, &yMax);
	seconds--;

	// Generate output
	int width = xMax - xMin + 1;
	int height = yMax - yMin + 1;

	char grid[height][width];
	memset(&(grid[0][0]), ' ', height * width * sizeof(char));

	int xOffset = xMin;
	int yOffset = yMin;

	for (int i = 0; i < LEN; ++i)
		grid[points[i].y - yOffset][points[i].x - xOffset] = '#';

	// Display output
	for (int y = 0; y < height; ++y) {
		for (int x = 0; x < width; ++x) {
			printf("%c", grid[y][x]);
		}
		printf("\n");
	}
	printf("Seconds: %d\n", seconds);
	
	return 0;
}
