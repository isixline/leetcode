var deckRevealedIncreasing = function(deck) {
  var sortedDeck = deck.sort((a, b) => a - b);
  var orderedDeck = [sortedDeck.pop()];
  while (sortedDeck.length > 0) {
    orderedDeck.unshift(sortedDeck.pop(), orderedDeck.pop());
  }
  return orderedDeck;
};
