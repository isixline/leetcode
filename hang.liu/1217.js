var minCostToMoveChips = function(chips) {
  var even = 0;
  var odd = 0;

  for (var chip of chips) {
    if (chip % 2 == 0) {
      even += 1;
    } else {
      odd += 1;
    }
  }

  return Math.min(even, odd);
};
