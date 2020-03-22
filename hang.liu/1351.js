var countNegatives = function(grid) {
  var m = grid.length;
  var n = grid[0].length;

  var result = 0;

  for (var i = 0; i < m; i++) {
    for (var j = 0; j < n; j++) {
      if (grid[i][j] < 0) {
        result += (n - j) * (m - i);
        n = j;
        break;
      }
    }
  }

  return result;
};
