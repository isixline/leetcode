var stoneGameII = function(piles) {
  var length = piles.length;
  var sum = new Array(length + 1).fill(0);
  
  var dp = new Array(length + 1);
  for (var i = 0; i <= length; i++) {
    dp[i] = new Array(length + 1).fill(0);
  }

  for (var i = length - 1; i >= 0; i--) {
    sum[i] = sum[i + 1] + piles[i];
    dp[i][length] = sum[i];
  }

  for (var i = length - 1; i >= 0; i--) {
    for (var m = length - 1; m >= 1; m--) {
      for (var x = 1; x <= 2 * m && i + x <= length; x++) {
        dp[i][m] = Math.max(dp[i][m], sum[i] - dp[i + x][Math.max(m, x)]);
      }
    }
  }

  return dp[0][1];
};
