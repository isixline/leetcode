var numRookCaptures = function(board) {
  var r = null;

  var row = board.length;
  var col = board[0].length;

  for (var i = 0; i < row && !r; i++) {
    for (var j = 0; j < col && !r; j++) {
      if (board[i][j] === "R") {
        r = [i, j];
      }
    }
  }

  var result = 0;
  var RRow = board[r[0]];
  var RCol = board.map(item => item[r[1]]);

  result += getPawns(RRow, r[1] - 1, 0, -1);
  result += getPawns(RRow, r[1] + 1, col - 1, 1);
  result += getPawns(RCol, r[0] - 1, 0, -1);
  result += getPawns(RCol, r[0] + 1, row - 1, 1);

  return result;
};

var getPawns = function(arr, start, end, step) {
  for (var i = start; i * step <= end * step; i += step) {
    if (arr[i] === "b" || arr[i] === "B") {
      return 0;
    }
    if (arr[i] === "p" || arr[i] === "P") {
      return 1;
    }
  }
  return 0;
};
