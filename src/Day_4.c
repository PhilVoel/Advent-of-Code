#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FIELD_SIZE 140

char *field[FIELD_SIZE];

bool has_up(uint8_t y, uint8_t x) {
	if (y>=3 &&
		field[y-1][x] == 'M' &&
		field[y-2][x] == 'A' &&
	 	field[y-3][x] == 'S')
		return true;
	return false;
}

bool has_up_left(uint8_t y, uint8_t x) {
	if (y>=3 && x>=3 &&
		field[y-1][x-1] == 'M' &&
		field[y-2][x-2] == 'A' &&
		field[y-3][x-3] == 'S')
		return true;
	return false;
}

bool has_left(uint8_t y, uint8_t x) {
	if (x>=3 &&
		field[y][x-1] == 'M' &&
		field[y][x-2] == 'A' &&
		field[y][x-3] == 'S')
		return true;
	return false;
}

bool has_down_left(uint8_t y, uint8_t x) {
	if (y<=FIELD_SIZE-4 && x>=3 &&
		field[y+1][x-1] == 'M' &&
		field[y+2][x-2] == 'A' &&
		field[y+3][x-3] == 'S')
		return true;
	return false;
}

bool has_down(uint8_t y, uint8_t x) {
	if (y<=FIELD_SIZE-4 &&
		field[y+1][x] == 'M' &&
		field[y+2][x] == 'A' &&
		field[y+3][x] == 'S')
		return true;
	return false;
}

bool has_down_right(uint8_t y, uint8_t x) {
	if (y<=FIELD_SIZE-4 && x<=FIELD_SIZE-4 &&
		field[y+1][x+1] == 'M' &&
		field[y+2][x+2] == 'A' &&
		field[y+3][x+3] == 'S')
		return true;
	return false;
}

bool has_right(uint8_t y, uint8_t x) {
	if (x<=FIELD_SIZE-4 &&
		field[y][x+1] == 'M' &&
		field[y][x+2] == 'A' &&
		field[y][x+3] == 'S')
		return true;
	return false;
}

bool has_up_right(uint8_t y, uint8_t x) {
	if (x<=FIELD_SIZE-4 && y>=3 &&
		field[y-1][x+1] == 'M' &&
		field[y-2][x+2] == 'A' &&
		field[y-3][x+3] == 'S')
		return true;
	return false;
}

int part1() {
	int count = 0;
	for(uint8_t i=0; i<FIELD_SIZE; i++) {
		for(uint8_t j=0; j<FIELD_SIZE; j++) {
			int count2 = count;
			if(field[i][j] != 'X') {
				printf(".");
				continue;
			}
			if(has_up(i, j)) count++;
			if(has_up_left(i, j)) count++;
			if(has_left(i, j)) count++;
			if(has_down_left(i, j)) count++;
			if(has_down(i, j)) count++;
			if(has_down_right(i, j)) count++;
			if(has_right(i, j)) count++;
			if(has_up_right(i, j)) count++;
			if(count2 == count) printf(".");
			else printf("%i", count-count2);
		}
		printf("\n");
	}
	return count;
}

int part2() {
	int count = 0;
	for(uint8_t i=1; i<FIELD_SIZE-1; i++) {
		for(uint8_t j=1; j<FIELD_SIZE-1; j++) {
			count += field[i][j] == 'A' &&
				(field[i-1][j-1] == 'M' && field[i+1][j+1] == 'S' ||
	  			field[i-1][j-1] == 'S' && field[i+1][j+1] == 'M') &&
				(field[i+1][j-1] == 'M' && field[i-1][j+1] == 'S' ||
	  			field[i+1][j-1] == 'S' && field[i-1][j+1] == 'M');
		}
	}
	return count;
}

int main(int argc, char *argv[]) {
	FILE *fp = fopen("inputs/4.txt", "r");
	size_t size=0;
	for(int i=0; i<FIELD_SIZE; i++) {
		if(getline(&field[i], &size, fp) == -1) {
			perror("Could not read line");
			return EXIT_FAILURE;
		}
	}
	fclose(fp);
	int count = argc==2 && !strcmp(argv[1], "--part2") ? part2() : part1();
	printf("%i\n", count);
	for(int i=0; i<FIELD_SIZE; i++) free(field[i]);
}
