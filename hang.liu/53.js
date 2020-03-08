var maxSubArray = function(nums) {
  var result = nums[0];
  var sums = nums.slice();

  var length = sums.length;

  for (var i = 1; i < length; i++) {
    sums[i] = Math.max(sums[i], sums[i] + sums[i - 1]);
    result = Math.max(result, sums[i]);
  }

  return result;
};
