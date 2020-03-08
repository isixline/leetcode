var climbStairs = function(n) {
  if (n === 1) return 1;
  if (n === 2) return 2;

  var result = 0;

  var oneStep = 1;
  var twoStep = 2;

  for (var i = 3; i <= n; i++) {
    result = oneStep + twoStep;
    oneStep = twoStep;
    twoStep = result;
  }

  return result;
};
