#include <algorithm>
#include <functional>
#include <iostream>
#include <fstream>
#include <string>

class Point {
public: 
	int x, y;

	Point(int x, int y) {
		x = x;
		y = y;
	};

	Point() {
		x = 0;
		y = 0;
	}
};

using namespace std;
int main() {

	// Read the file
	ifstream contents;
	string line;
	contents.open("input.txt", ios::in);
	if (!contents) { 
		cout << "Could not find file" << endl;
		return 0;
	}

	int grid_width = 0;
	int grid_height = 0;
	vector<vector<char> > grid;
	
	while (getline(contents, line)) {
		vector<char> new_row;
		if (grid_width == 0) { grid_width = line.length(); }
		for (int i = 0; i < line.length(); i++) {
			//cout << line[i];
			new_row.push_back(line[i]);
		}
		//cout << endl;
		grid_height += 1;
		grid.push_back(new_row);
	}
	cout << "Done reading lines" << endl;
	cout << "Width: " << grid_width << endl;
	cout << "Height: " << grid_height << endl;

	// Get all offset locations 
	vector<int> x_offsets;
	vector<int> y_offsets;

	for (int j = 0; j < grid_height; j++) {
		// Check if all of the elements in the row are equal
		if (adjacent_find( grid[j].begin(), grid[j].end(), std::not_equal_to<char>() ) == grid[j].end() ) {
			y_offsets.push_back(j);
		}
	}

	for (int i = 0; i < grid_width; i++) {
		// Make a vertical row for scanning
		vector<char> column;
		for (int j = 0; j < grid_height; j++) {
			column.push_back(grid[j][i]);
		}
		// Check if all of the elements in the col are equal
		if (adjacent_find( column.begin(), column.end(), std::not_equal_to<char>() ) == column.end() ) {
			x_offsets.push_back(i);
		}
	}

	vector<Point> star_locations;
	vector<int> working_x_offsets = x_offsets;
	vector<int> working_y_offsets = y_offsets;

	long total_x_offset = 0;
	long total_y_offset = 0;

	for (int j = 0; j < grid_height; j++) {
		// Reset x offsets for each line
		working_x_offsets = x_offsets;
		total_x_offset = 0;
		for (int i = 0; i < grid_width; i++) {
			//cout << "============ At: (" << i << ", " << j << ")." << endl;
			if (working_x_offsets.size() > 0 && i == working_x_offsets[0] ) {
				total_x_offset += 1;
				working_x_offsets.erase(working_x_offsets.begin());
			}
			if (working_y_offsets.size() > 0 && j == working_y_offsets[0] ) {
				total_y_offset += 1;
				working_y_offsets.erase(working_y_offsets.begin());
			}
			if (grid[j][i] == '#') {
				Point *p = new Point; 
				p->x = i + total_x_offset;
				p->y = j + total_y_offset;
				star_locations.push_back(*p);

				// This is just for visualization
				// grid[j][i] = '0' + star_locations.size();
			}
		}
	}

	long total = 0;

	cout << "Number of stars " << star_locations.size() << endl;
	// For each node, get the distance to every other node, and add it to the total
	for (int j = 0; j < star_locations.size(); j++ ) {
		for (int i = j + 1; i < star_locations.size(); i++ ) {
			int x_dist = abs(star_locations[j].x - star_locations[i].x);
			int y_dist = abs(star_locations[j].y - star_locations[i].y);
			total += x_dist + y_dist;
		}
	}
	
	cout << total << endl;

	return 0;
}
