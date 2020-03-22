var getDecimalValue = function(head) {
  var result = 0;
  var current = head;

  while (current) {
    result = result * 2 + current.val;
    current = current.next;
  }

  return result;
};
