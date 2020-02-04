var findNumbers = function(nums) {
    return nums.map(e => e.toString().length % 2 === 0 ? 1 : 0)
               .reduce((a, c) => a + c, 0);
};
