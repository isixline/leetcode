var middleNode = function(head) {
  var current = head;
  var fastCurrent = head;

  while (fastCurrent && fastCurrent.next) {
    current = current.next;
    fastCurrent = fastCurrent.next.next;
  }

  return current;
};
