var relativeSortArray = function(arr1, arr2) {
  return arr1.sort((a, b) => {
    var indexA = arr2.indexOf(a);
    var indexB = arr2.indexOf(b);

    if (indexA < 0 && indexB < 0) return a - b;
    if (indexA < 0) return 1;
    if (indexB < 0) return -1;
    return indexA - indexB;
  });
};
