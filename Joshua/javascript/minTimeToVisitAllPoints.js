var minTimeToVisitAllPoints = function(points) {
    function moveCountBetweentPoint(p1, p2) {
        let distanceX = Math.abs(p1[0] - p2[0]);
        let distanceY = Math.abs(p1[1] - p2[1]);

        let slantingMove = Math.min(distanceX, distanceY);
        let axisMove = Math.abs(distanceX - distanceY);

        return slantingMove + axisMove;
    }
    
    let totalMove = 0;
    
    for (let i = 0; i < points.length - 1; i++) {
        totalMove += moveCountBetweentPoint(points[i], points[i + 1]);
    }
    return totalMove;
};