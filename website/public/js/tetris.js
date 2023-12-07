document.addEventListener('DOMContentLoaded', function() {
    const canvas = document.getElementById('tetrisCanvas');
    const context = canvas.getContext('2d');
    context.scale(20, 20);

    function drawMatrix(matrix, offset) {
        matrix.forEach((row, y) => {
            row.forEach((value, x) => {
                if (value !== 0) {
                    context.fillStyle = 'red';
                    context.fillRect(x + offset.x, y + offset.y, 1, 1);
                }
            });
        });
    }

    const socket = new WebSocket('ws://127.0.0.1:8080/ws/');

    socket.addEventListener('open', function(event) {
        console.log("Connecté au serveur WebSocket");
    });

    socket.addEventListener('message', function(event) {
        const data = JSON.parse(event.data);
        console.log('Message from server ', data);
        switch(data.type) {
            case 'matrix':
                updateMatrix(data.matrix);
                break;
        }
    });

    function updateMatrix(matrix) {
        context.clearRect(0, 0, canvas.width, canvas.height);
        matrix.forEach((row, y) => {
            row.forEach((value, x) => {
                if (value !== 0) {
                    context.fillStyle = 'red';
                    context.fillRect(x, y, 1, 1);
                }
            });
        });
    }

    document.addEventListener('keydown', event => {
        socket.send(JSON.stringify({ type: 'move', direction: event.key }));
        console.log(event.key);
    });
});
