var commonChars = function(A) {
  var calculateCharCount = s => {
    var charCount = Array(26).fill(0);
    for (var i = 0; i < s.length; i++) {
      charCount[s.charCodeAt(i) - 97] += 1;
    }
    return charCount;
  };
  var charCount = calculateCharCount(A[0]);

  for (var i = 1; i < A.length; i++) {
    var temp = calculateCharCount(A[i]);
    for (var j = 0; j < 26; j++) {
      charCount[j] = Math.min(charCount[j], temp[j]);
    }
  }

  var result = [];

  for (var i = 0; i < 26; i++) {
    for (var j = 0; j < charCount[i]; j++) {
      result.push(String.fromCharCode(i + 97));
    }
  }

  return result;
};
