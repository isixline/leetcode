var countSquares = function(matrix) {
  var m = matrix.length;
  var n = matrix[0].length;
  var result = 0;

  for (var i = 0; i < m; i++) {
    for (var j = 0; j < n; j++) {
      if (matrix[i][j] > 0 && i > 0 && j > 0) {
        matrix[i][j] =
          Math.min(
            matrix[i - 1][j - 1],
            Math.min(matrix[i - 1][j], matrix[i][j - 1])
          ) + 1;
      }
      result += matrix[i][j];
    }
  }

  return result;
};
