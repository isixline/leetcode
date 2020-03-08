var maxProfit = function(prices) {
  var result = 0;

  var length = prices.length;

  for (var i = 1; i < length; i++) {
    const profit = prices[i] - prices[i - 1];
    prices[i] = Math.min(prices[i], prices[i - 1]);

    if (profit > result) result = profit;
  }

  return result;
};
