var luckyNumbers = function(matrix) {
  var m = matrix.length;
  var n = matrix[0].length;
  var result = [];

  for (var i = 0; i < m; i++) {
    let minIndex = 0;
    for (var j = 1; j < n; j++) {
      if (matrix[i][j] < matrix[i][minIndex]) minIndex = j;
    }

    let isLuck = true;
    for (var j = 0; j < m; j++) {
      if (matrix[j][minIndex] > matrix[i][minIndex]) {
        isLuck = false;
        break;
      }
    }

    if (isLuck) result.push(matrix[i][minIndex]);
  }

  return result;
};
