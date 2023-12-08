document.addEventListener('DOMContentLoaded', function() {
    const canvas = document.getElementById('tetrisCanvas');
    const context = canvas.getContext('2d');

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
        const cellSize = 20;
        const cellBorder = 2;
        matrix.forEach((row, y) => {
            row.forEach((value, x) => {
                if (value === 0) {
                    return;
                }
                if (value === 1) {
                    context.fillStyle = 'red';
                }
                if (value === 2) {
                    context.fillStyle = 'blue';
                }
                if (value === 3) {
                    context.fillStyle = 'yellow';
                }
                if (value === 4) {
                    context.fillStyle = 'green';
                }
                if (value === 5) {
                    context.fillStyle = 'purple';
                }
                if (value === 6) {
                    context.fillStyle = 'orange';
                }
                if (value === 7) {
                    context.fillStyle = 'pink';
                }
                let x_coord = x * cellSize + cellBorder;
                let y_coord = y * cellSize + cellBorder;
                context.fillRect(x_coord, y_coord, cellSize - cellBorder * 2, cellSize - cellBorder * 2);
            });
        });
    }

    document.addEventListener('keydown', event => {
        socket.send(JSON.stringify({ type: 'move', direction: event.key }));
        console.log(event.key);
    });
});
