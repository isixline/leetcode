var dayOfTheWeek = function(day, month, year) {
  const weekMap = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
  ];
  // return weekMap[new Date(year, month - 1, day).getDay()]

  var isLeap = year => (year % 4 === 0 && year % 100 !== 0) || year % 400 == 0;
  var months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  var days = 0;

  for (var i = 1971; i < year; i++) {
    days += 365;
    if (isLeap(i)) days += 1;
  }

  for (var i = 0; i < month - 1; i++) {
    days += months[i];
  }

  if (month > 2 && isLeap(year)) days += 1;

  days += day;

  return weekMap[(5 - 1 + days) % 7];
};
