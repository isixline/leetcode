var isSubsequence = function(s, t) {
  var index = 0;
  var sLength = s.length;
  var tLength = t.length;

  for (var i = 0; i < tLength && index < sLength; i++) {
    if (s[index] === t[i]) index++;
  }

  return index === sLength;
};
