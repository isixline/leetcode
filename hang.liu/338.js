var countBits = function(num) {
  if (num === 0) return [0];
  if (num === 1) return [0, 1];
  if (num === 2) return [0, 1, 1];
  if (num === 3) return [0, 1, 1, 2];
  var result = [0, 1, 1, 2];

  var index = 2;

  var length = Math.ceil(num / 2) * 2;

  for (var i = 4; i <= length; i *= 2) {
    var temp = result.slice(index, i);
    result.push(...temp);
    result.push(...temp.map(item => item + 1));
    index = i;
  }

  return result.slice(0, num + 1);
};
