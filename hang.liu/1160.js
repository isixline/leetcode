var countCharacters = function(words, chars) {
  var result = 0;
  var sortedChars = chars
    .split("")
    .sort()
    .join("");

  for (var word of words) {
    var sortedWord = word
      .split("")
      .sort()
      .join("");
    var indexWord = 0;
    var indexChars = 0;
    var lengthWord = sortedWord.length;
    var lengthChars = sortedChars.length;
    while (indexWord < lengthWord && indexChars < lengthChars) {
      if (sortedWord[indexWord] === sortedChars[indexChars]) {
        indexWord++;
        indexChars++;
      } else {
        indexChars++;
      }
    }

    if (indexWord >= lengthWord) result += lengthWord;
  }

  return result;
};
