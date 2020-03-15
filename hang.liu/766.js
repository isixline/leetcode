var isToeplitzMatrix = function(matrix) {
  var m = matrix.length;
  var n = matrix[0].length;
  for (var i = 0; i < m; i++) {
    for (var j = 0; j < n; j++) {
      if (i - 1 >= 0 && j - 1 >= 0 && matrix[i - 1][j - 1] !== matrix[i][j]) {
        return false;
      }
    }
  }

  return true;
};
