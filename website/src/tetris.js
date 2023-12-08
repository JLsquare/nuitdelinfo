import '../style/style.css';

document.addEventListener('DOMContentLoaded', function() {
    const playerCanvas = document.getElementById('playerCanvas');
    const context = playerCanvas.getContext('2d');
    const scoreElement = document.getElementById('score');

    let playerId = null;
    let enemyTimeouts = {};

    const socket = new WebSocket(`ws://${window.location.host}/ws/`);

    socket.addEventListener('open', function(event) {
        console.log("Connecté au serveur WebSocket");
    });

    socket.addEventListener('message', function(event) {
        const data = JSON.parse(event.data);
        console.log('Message from server ', data);
        switch(data.type) {
            case 'matrix':
                updateMatrix(context, playerCanvas, data.matrix);
                break;
            case 'score':
                scoreElement.innerHTML = data.score;
                break;
            case 'broadcast_matrix':
                if (data.id !== playerId) {
                    updateEnemyMatrix(data.id, data.matrix);
                }
                break;
            case 'id':
                playerId = data.id;
                break;
            case 'finished':
                playerCanvas.classList.add('hidden');
                if (data.score >= 4000) {
                    document.getElementById('win').classList.remove('hidden');
                    document.getElementById('lose').classList.add('hidden');
                } else {
                    document.getElementById('lose').classList.remove('hidden');
                    document.getElementById('win').classList.add('hidden');
                }
                break;
        }
    });

    function updateMatrix(context, canvas, matrix) {
        drawMatrix(context, canvas, matrix);
    }

    function updateEnemyMatrix(id, matrix) {
        let enemyCanvas = document.getElementById(`enemyCanvas-${id}`);
        if (!enemyCanvas) {
            enemyCanvas = createEnemyCanvas(id);
        }
        let enemyContext = enemyCanvas.getContext('2d');
        drawMatrix(enemyContext, enemyCanvas, matrix);

        clearTimeout(enemyTimeouts[id]);
        enemyTimeouts[id] = setTimeout(() => removeEnemy(id), 2000);
    }

    function removeEnemy(id) {
        let enemyCanvas = document.getElementById(`enemyCanvas-${id}`);
        if (enemyCanvas) {
            enemyCanvas.parentNode.removeChild(enemyCanvas);
        }
        delete enemyTimeouts[id];
    }

    function createEnemyCanvas(id) {
        const totalEnemyCanvases = document.querySelectorAll('[id^="enemyCanvas-"]').length;

        if (totalEnemyCanvases >= 4) {
            return;
        }

        let newCanvas = document.createElement('canvas');
        newCanvas.id = `enemyCanvas-${id}`;
        newCanvas.width = 200;
        newCanvas.height = 400;
        newCanvas.className = 'border border-gray-400';

        let leftGames = document.getElementById('leftGames');
        let rightGames = document.getElementById('rightGames');

        if (leftGames.childElementCount <= rightGames.childElementCount) {
            document.getElementById('leftGames').appendChild(newCanvas);
        } else {
            document.getElementById('rightGames').appendChild(newCanvas);
        }

        return newCanvas;
    }


    function drawMatrix(context, canvas, matrix) {
        context.clearRect(0, 0, canvas.width, canvas.height);
        const cellSize = 20;
        const cellBorder = 2;
        matrix.forEach((row, y) => {
            row.forEach((value, x) => {
                if (value === 0) {
                    return;
                }
                context.fillStyle = getColor(value);
                let x_coord = x * cellSize + cellBorder;
                let y_coord = y * cellSize + cellBorder;
                context.fillRect(x_coord, y_coord, cellSize - cellBorder * 2, cellSize - cellBorder * 2);
            });
        });
    }

    function getColor(value) {
        const colors = ['red', 'blue', 'yellow', 'green', 'purple', 'orange', 'pink'];
        return colors[value - 1] || 'black';
    }

    document.addEventListener('keydown', event => {
        socket.send(JSON.stringify({ type: 'move', direction: event.key }));
        console.log(event.key);
    });
});
